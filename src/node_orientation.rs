use bevy::prelude::*;


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
            left: node.bottom,
            right: node.top,
            top: node.left,
            bottom: node.right,
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::Column,
                FlexDirection::Column => FlexDirection::RowReverse,
                FlexDirection::RowReverse => FlexDirection::ColumnReverse,
                FlexDirection::ColumnReverse => FlexDirection::Row
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
            width: node.height,
            height: node.width,
            min_width: node.min_height,
            min_height: node.min_width,
            max_width: node.max_height,
            max_height: node.max_width,
            aspect_ratio: node.aspect_ratio.map(|r| 1./r),
            overflow: Overflow { x: node.overflow.y, y: node.overflow.x },
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
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::RowReverse,
                FlexDirection::Column => FlexDirection::ColumnReverse,
                FlexDirection::RowReverse => FlexDirection::Row,
                FlexDirection::ColumnReverse => FlexDirection::Column
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
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
            left: node.top,
            right: node.bottom,
            top: node.right,
            bottom: node.left,
            flex_direction: match node.flex_direction {
                FlexDirection::Row => FlexDirection::ColumnReverse,
                FlexDirection::Column => FlexDirection::Row,
                FlexDirection::RowReverse => FlexDirection::Column,
                FlexDirection::ColumnReverse => FlexDirection::RowReverse
            },
            margin: rotate_uirect(node.margin),
            padding: rotate_uirect(node.padding),
            border: rotate_uirect(node.border),
            width: node.height,
            height: node.width,
            min_width: node.min_height,
            min_height: node.min_width,
            max_width: node.max_height,
            max_height: node.max_width,
            aspect_ratio: node.aspect_ratio.map(|r| 1./r),
            overflow: Overflow { x: node.overflow.y, y: node.overflow.x },
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
}