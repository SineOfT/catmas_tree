use bevy::prelude::*;
use std::{
    error::Error,
    fmt,
    marker::PhantomData
};


#[derive(Debug)]
enum NodeSizeError {
    NotEnoughValues,
    NotEnoughOperators,
    Unreachable
}

impl Error for NodeSizeError { }

impl fmt::Display for NodeSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughValues => write!(f, "Not enough values passed"),
            Self::NotEnoughOperators => write!(f, "Not enough operators passed"),
            Self::Unreachable => write!(f, "This error should be unreachable, logic bug")
        }
    }
}


#[derive(Resource)]
pub struct RRem(pub f32);

#[derive(Component)]
pub struct CEmSize(pub f32);


pub struct NodeWidth;
pub struct NodeHeight;
pub struct NodeJustify;
pub struct NodeAlign;
pub struct NodeFontSize;

use sealed::NodeSizeMarker;

mod sealed {
    use super::*;

    pub trait NodeSizeMarker: Sync + Send + 'static {
        type NodeType: Component;

        fn update_node_size(node: &mut Self::NodeType, size: f32);
    }

    impl NodeSizeMarker for NodeWidth {
        type NodeType = Node;
        fn update_node_size(node: &mut Node, size: f32) { node.width = Val::Px(size); }
    }
    impl NodeSizeMarker for NodeHeight {
        type NodeType = Node;
        fn update_node_size(node: &mut Node, size: f32) { node.height = Val::Px(size); }
    }
    impl NodeSizeMarker for NodeJustify {
        type NodeType = Node;
        fn update_node_size(node: &mut Node, size: f32) {
            match node.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => node.width = Val::Px(size),
                FlexDirection::Column | FlexDirection::ColumnReverse => node.height = Val::Px(size),
            };
        }
    }
    impl NodeSizeMarker for NodeAlign {
        type NodeType = Node;
        fn update_node_size(node: &mut Node, size: f32) {
            match node.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => node.height = Val::Px(size),
                FlexDirection::Column | FlexDirection::ColumnReverse => node.width = Val::Px(size),
            };
        }
    }
    impl NodeSizeMarker for NodeFontSize {
        type NodeType = TextFont;
        fn update_node_size(node: &mut TextFont, size: f32) { node.font_size = size; }
    }
}


#[derive(Clone)]
pub enum NodeSizeOperator {
    Rem(f32),
    Em(f32),
    Add,
    Subtract,
    Multiply,
    Divide,
    Max,
    Min
}

#[derive(Component, Deref)]
pub struct CNodeSize<M: NodeSizeMarker> {
    #[deref]
    operators: Vec<NodeSizeOperator>,
    phantom: PhantomData<M>
}

impl<M: NodeSizeMarker> Clone for CNodeSize<M> {
    fn clone(&self) -> Self {
        Self {
            operators: self.operators.clone(),
            phantom: PhantomData
        }
    }
}

impl<M: NodeSizeMarker> CNodeSize<M> {
    const SIZE_IF_ERROR: f32 = 100.;
    
    pub fn new(operator: NodeSizeOperator) -> Self {
        Self { operators: vec![operator], phantom: PhantomData }
    }
    
    pub fn calc(operators: Vec<NodeSizeOperator>) -> Self {
        Self { operators, phantom: PhantomData }
    }

    pub fn operators(&self) -> &Vec<NodeSizeOperator> { &self.operators }

    pub fn clone_as<N: NodeSizeMarker>(&self) -> CNodeSize<N> {
        CNodeSize::<N> { operators: self.operators.clone(), phantom: PhantomData }
    }

    pub fn update_node_size(
        &self,
        entity: Entity,
        node: &mut M::NodeType,
        rem: &RRem,
        parent_query: &Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {

        let size = match self.calculate(entity, rem, parent_query) {
            Ok(size) => size,
            Err(e) => {
                warn!("{e}");
                Self::SIZE_IF_ERROR
            }
        };

        M::update_node_size(node, size);
    }
    
    fn calculate(
        &self,
        entity: Entity,
        rem: &RRem,
        parent_query: &Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) -> Result<f32, NodeSizeError> {

        fn calculate_rem(result_stack: &mut Vec<f32>, rem_size: f32, percent: f32) {
            result_stack.push(rem_size * percent)
        }
        fn calculate_em(
            result_stack: &mut Vec<f32>,
            em_size: &mut Option<f32>,
            rem_size: f32,
            percent: f32,
            entity: Entity,
            parent_query: &Query<(&Parent, Option<&CEmSize>), With<Node>>
        ) {
            
            fn find_em_size(entity: Entity, rem_size: f32, parent_query: &Query<(&Parent, Option<&CEmSize>), With<Node>>) -> f32 {
                match parent_query.get(entity) {
                    Ok((parent_entity, has_em_size)) => (
                        match has_em_size {
                            Some(em_size) => em_size.0,
                            None => 1.
                        } * find_em_size(parent_entity.get(), rem_size, parent_query)
                    ),
                    Err(_) => rem_size
                }
            }

            match em_size {
                Some(em_size) => {
                    result_stack.push(*em_size * percent)
                },
                None => {
                    let em = find_em_size(entity, rem_size, parent_query);
                    *em_size = Some(em);
                    result_stack.push(em * percent);
                }
            }
        }
        fn calculate_add(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(a + b);
            Ok(())
        }
        fn calculate_subtract(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(a - b);
            Ok(())
        }
        fn calculate_multiply(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(a * b);
            Ok(())
        }
        fn calculate_divide(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(a / b);
            Ok(())
        }
        fn calculate_min(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(f32::min(a, b));
            Ok(())
        }
        fn calculate_max(result_stack: &mut Vec<f32>) -> Result<(), NodeSizeError> {
            let Some(b) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
            let Some(a) = result_stack.pop() else { return Err(NodeSizeError::NotEnoughValues); };
            result_stack.push(f32::max(a, b));
            Ok(())
        }

        let mut result_stack = Vec::with_capacity(self.len());

        let rem_size = rem.0;
        let mut em_size = None;

        for operator in self.iter() {
            match operator {
                NodeSizeOperator::Rem(percent) => calculate_rem(&mut result_stack, rem_size, *percent),
                NodeSizeOperator::Em(percent) => calculate_em(&mut result_stack, &mut em_size, rem_size, *percent, entity, &parent_query),
                NodeSizeOperator::Add => calculate_add(&mut result_stack)?,
                NodeSizeOperator::Subtract => calculate_subtract(&mut result_stack)?,
                NodeSizeOperator::Multiply => calculate_multiply(&mut result_stack)?,
                NodeSizeOperator::Divide => calculate_divide(&mut result_stack)?,
                NodeSizeOperator::Max => calculate_max(&mut result_stack)?,
                NodeSizeOperator::Min => calculate_min(&mut result_stack)?
            };
        }

        let Some(result) = result_stack.pop() else { return Err(NodeSizeError::Unreachable); };
        if !result_stack.is_empty() { return Err(NodeSizeError::NotEnoughOperators); };

        Ok(result)
    }
}


pub mod update {
    use super::*;


    pub fn update_node_sizes<M: NodeSizeMarker>(
        rem: Res<RRem>,
        mut node_query: Query<(Entity, &mut M::NodeType, &CNodeSize<M>)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {

        for (entity, mut node, node_size) in node_query.iter_mut() {
            node_size.update_node_size(entity, &mut node, &rem, &parent_query);
        }
    }
}