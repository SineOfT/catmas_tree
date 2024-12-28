use bevy::prelude::*;

use super::rem::RRem;



#[derive(Component)]
pub struct CEmWidth(pub f32);
impl CEmWidth { fn to_val(&self, em_size: f32) -> Val { Val::Px(self.0 * em_size) } }

#[derive(Component)]
pub struct CEmHeight(pub f32);
impl CEmHeight { fn to_val(&self, em_size: f32) -> Val { Val::Px(self.0 * em_size) } }

#[derive(Component)]
pub struct CEmJustifySize(pub f32);
impl CEmJustifySize { fn to_val(&self, em_size: f32) -> Val { Val::Px(self.0 * em_size) } }

#[derive(Component)]
pub struct CEmAlignSize(pub f32);
impl CEmAlignSize { fn to_val(&self, em_size: f32) -> Val { Val::Px(self.0 * em_size) } }

#[derive(Component)]
pub struct CEmFontSize(pub f32);
impl CEmFontSize { fn to_val(&self, em_size: f32) -> f32 { self.0 * em_size } }


#[derive(Component)]
pub struct CEmSize(pub f32);



pub mod update {
    use super::*;


    fn find_em_size(entity: Entity, rem: &RRem, parent_query: &Query<(&Parent, Option<&CEmSize>), With<Node>>) -> f32 {
        match parent_query.get(entity) {
            Ok((parent_entity, has_em_size)) => (
                match has_em_size {
                    Some(em_size) => em_size.0,
                    None => 1.
                } * find_em_size(parent_entity.get(), rem, parent_query)
            ),
            Err(_) => rem.0
        }
    }


    pub fn update_widths(
        rem: Res<RRem>,
        mut node_query: Query<(Entity, &mut Node, &CEmWidth)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {

        for (entity, mut node, width) in node_query.iter_mut() {
            let em_size = find_em_size(entity, &rem, &parent_query);
            node.width = width.to_val(em_size);
            info!("{:?}", node.width);
        }
    }

    pub fn update_heights(
        rem: Res<RRem>,
        mut node_query: Query<(Entity, &mut Node, &CEmHeight)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {
        
        for (entity, mut node, height) in node_query.iter_mut() {
            let em_size = find_em_size(entity, &rem, &parent_query);
            node.height = height.to_val(em_size);
        }
    }

    pub fn update_justify_sizes(
        rem: Res<RRem>,
        mut node_query: Query<(Entity, &mut Node, &CEmJustifySize)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {
        
        for (entity, mut node, justify_size) in node_query.iter_mut() {
            let em_size = find_em_size(entity, &rem, &parent_query);
            match node.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => { node.width = justify_size.to_val(em_size) },
                FlexDirection::Column | FlexDirection::ColumnReverse => { node.height = justify_size.to_val(em_size) }
            }
        }
    }

    pub fn update_align_sizes(
        rem: Res<RRem>,
        mut node_query: Query<(Entity, &mut Node, &CEmAlignSize)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {
        
        for (entity, mut node, align_size) in node_query.iter_mut() {
            let em_size = find_em_size(entity, &rem, &parent_query);
            match node.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => { node.height = align_size.to_val(em_size) },
                FlexDirection::Column | FlexDirection::ColumnReverse => { node.width = align_size.to_val(em_size) }
            }
        }
    }

    pub fn update_font_sizes(
        rem: Res<RRem>,
        mut text_font_query: Query<(Entity, &mut TextFont, &CEmFontSize)>,
        parent_query: Query<(&Parent, Option<&CEmSize>), With<Node>>
    ) {

        for (entity, mut text_font, font_size) in text_font_query.iter_mut() {
            let em_size = find_em_size(entity, &rem, &parent_query);
            text_font.font_size = font_size.to_val(em_size);
        }
    }
}