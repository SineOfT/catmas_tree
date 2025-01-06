use bevy::prelude::*;

use super::selectable_ui::grid::GridDimension;


#[derive(Clone, Copy)]
pub enum NodeOrientation {
    Zero,
    OneForth,
    OneHalf,
    ThreeForth
}

impl NodeOrientation {
    pub fn apply(&self, node: Node) -> Node {
        match self {
            Self::Zero => node,
            Self::OneForth => self.rotate_one_forth(node),
            Self::OneHalf => self.rotate_one_half(node),
            Self::ThreeForth => self.rotate_three_forth(node)
        }
    }

    fn rotate_one_forth(&self, node: Node) -> Node {
        fn rotate_uirect(ui_rect: UiRect) -> UiRect {
            UiRect {
                left: ui_rect.bottom,
                right: ui_rect.top,
                top: ui_rect.left,
                bottom: ui_rect.right
            }
        }
        
        Node {
            overflow: Overflow { x: node.overflow.y, y: node.overflow.x },
            left: node.bottom,
            right: node.top,
            top: node.left,
            bottom: node.right,
            width: node.height,
            height: node.width,
            min_width: node.min_height,
            min_height: node.min_width,
            max_width: node.max_height,
            max_height: node.max_width,
            aspect_ratio: node.aspect_ratio.map(|r| 1./r),
            align_items: match node.align_items {
                AlignItems::Start | AlignItems::FlexStart | AlignItems::End | AlignItems::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignItems::FlexEnd,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignItems::FlexStart
                },
                align_items => align_items
            },
            align_self: match node.align_self {
                AlignSelf::Start | AlignSelf::FlexStart | AlignSelf::End | AlignSelf::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignSelf::FlexEnd,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignSelf::FlexStart
                },
                align_self => align_self
            },
            align_content: match node.align_content {
                AlignContent::Start | AlignContent::FlexStart | AlignContent::End | AlignContent::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignContent::FlexEnd,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignContent::FlexStart
                },
                align_content => align_content
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::Column,
                FlexDirection::Column => FlexDirection::RowReverse,
                FlexDirection::RowReverse => FlexDirection::ColumnReverse,
                FlexDirection::ColumnReverse => FlexDirection::Row
            },
            row_gap: node.column_gap,
            column_gap: node.row_gap,
            grid_auto_flow: match node.grid_auto_flow {
                GridAutoFlow::Row => GridAutoFlow::Column,
                GridAutoFlow::Column => GridAutoFlow::Row,
                GridAutoFlow::RowDense => GridAutoFlow::ColumnDense,
                GridAutoFlow::ColumnDense => GridAutoFlow::RowDense
            },
            grid_template_rows: node.grid_template_columns,
            grid_template_columns: node.grid_template_rows,
            grid_auto_rows: node.grid_auto_columns,
            grid_auto_columns: node.grid_auto_rows,
            grid_row: node.grid_column,
            grid_column: node.grid_row,
            ..node
        }
    }

    fn rotate_one_half(&self, node: Node) -> Node {
        fn rotate_uirect(ui_rect: UiRect) -> UiRect {
            UiRect {
                left: ui_rect.right,
                right: ui_rect.left,
                top: ui_rect.bottom,
                bottom: ui_rect.top
            }
        }
        
        Node {
            left: node.right,
            right: node.left,
            top: node.bottom,
            bottom: node.top,
            align_items: match node.align_items {
                AlignItems::Start | AlignItems::FlexStart => AlignItems::FlexEnd,
                AlignItems::End | AlignItems::FlexEnd => AlignItems::FlexStart,
                align_items => align_items
            },
            align_self: match node.align_self {
                AlignSelf::Start | AlignSelf::FlexStart => AlignSelf::FlexEnd,
                AlignSelf::End | AlignSelf::FlexEnd => AlignSelf::FlexStart,
                align_self => align_self
            },
            align_content: match node.align_content {
                AlignContent::Start | AlignContent::FlexStart => AlignContent::FlexEnd,
                AlignContent::End | AlignContent::FlexEnd => AlignContent::FlexStart,
                align_content => align_content
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::RowReverse,
                FlexDirection::Column => FlexDirection::ColumnReverse,
                FlexDirection::RowReverse => FlexDirection::Row,
                FlexDirection::ColumnReverse => FlexDirection::Column
            },
            ..node
        }
    }

    fn rotate_three_forth(&self, node: Node) -> Node {
        fn rotate_uirect(ui_rect: UiRect) -> UiRect {
            UiRect {
                left: ui_rect.top,
                right: ui_rect.bottom,
                top: ui_rect.right,
                bottom: ui_rect.left
            }
        }
        
        Node {
            overflow: Overflow { x: node.overflow.y, y: node.overflow.x },
            left: node.top,
            right: node.bottom,
            top: node.right,
            bottom: node.left,
            width: node.height,
            height: node.width,
            min_width: node.min_height,
            min_height: node.min_width,
            max_width: node.max_height,
            max_height: node.max_width,
            aspect_ratio: node.aspect_ratio.map(|r| 1./r),
            align_items: match node.align_items {
                AlignItems::Start | AlignItems::FlexStart | AlignItems::End | AlignItems::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignItems::FlexStart,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignItems::FlexEnd
                },
                align_items => align_items
            },
            align_self: match node.align_self {
                AlignSelf::Start | AlignSelf::FlexStart | AlignSelf::End | AlignSelf::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignSelf::FlexStart,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignSelf::FlexEnd
                },
                align_self => align_self
            },
            align_content: match node.align_content {
                AlignContent::Start | AlignContent::FlexStart | AlignContent::End | AlignContent::FlexEnd => match node.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => AlignContent::FlexStart,
                    FlexDirection::Column | FlexDirection::ColumnReverse => AlignContent::FlexEnd
                },
                align_content => align_content
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::ColumnReverse,
                FlexDirection::Column => FlexDirection::Row,
                FlexDirection::RowReverse => FlexDirection::Column,
                FlexDirection::ColumnReverse => FlexDirection::RowReverse
            },
            row_gap: node.column_gap,
            column_gap: node.row_gap,
            grid_auto_flow: match node.grid_auto_flow {
                GridAutoFlow::Row => GridAutoFlow::Column,
                GridAutoFlow::Column => GridAutoFlow::Row,
                GridAutoFlow::RowDense => GridAutoFlow::ColumnDense,
                GridAutoFlow::ColumnDense => GridAutoFlow::RowDense
            },
            grid_template_rows: node.grid_template_columns,
            grid_template_columns: node.grid_template_rows,
            grid_auto_rows: node.grid_auto_columns,
            grid_auto_columns: node.grid_auto_rows,
            grid_row: node.grid_column,
            grid_column: node.grid_row,
            ..node
        }
    }


    pub fn apply_to_grid_dimensions<const N: usize>(&self, grid_dimensions: [GridDimension; N]) -> [GridDimension; N] {
        match self {
            Self::Zero => grid_dimensions,
            Self::OneForth => grid_dimensions.map(|dimension| match dimension {
                GridDimension::X => GridDimension::Y,
                GridDimension::Y => GridDimension::NegX,
                GridDimension::NegX => GridDimension::NegY,
                GridDimension::NegY => GridDimension::X,
                dimension => dimension
            }),
            Self::OneHalf => grid_dimensions.map(|dimension| match dimension {
                GridDimension::X => GridDimension::NegX,
                GridDimension::Y => GridDimension::NegY,
                GridDimension::NegX => GridDimension::X,
                GridDimension::NegY => GridDimension::Y,
                dimension => dimension
            }),
            Self::ThreeForth => grid_dimensions.map(|dimension| match dimension {
                GridDimension::X => GridDimension::NegY,
                GridDimension::Y => GridDimension::X,
                GridDimension::NegX => GridDimension::Y,
                GridDimension::NegY => GridDimension::NegX,
                dimension => dimension
            })
        }
    }


    pub fn apply_to_size(&self, size: Vec2) -> Vec2 {
        match self {
            Self::Zero | Self::OneHalf => size,
            Self::OneForth | Self::ThreeForth => size.yx()
        }
    }


    pub fn set_justify(node: &mut Node, justify: Val) {
        match node.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                node.width = justify;
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                node.height = justify;
            }
        }
    }

    pub fn set_align(node: &mut Node, align: Val) {
        match node.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                node.height = align;
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                node.width = align;
            }
        }
    }
}