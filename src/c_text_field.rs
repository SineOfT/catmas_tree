use bevy::{
    prelude::*,
    input::{
        ButtonState,
        keyboard::{KeyboardInput, Key}
    }
};

use super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeFontSize},
    selectable_ui::{}
};


const FONT_PATH: &'static str = "source_code_pro.ttf";
const CHARACTER_WIDTH: f32 = 0.6;


#[derive(Component)]
pub struct CTextField {
    char_count: u8
}

impl CTextField {
    pub fn new(char_count: u8) -> Self {
        Self { char_count }
    }

    pub fn build(
        commands: &mut Commands,
        asset_server: &AssetServer,
        text_field: CTextField
    ) -> Entity {

        commands
            .spawn((
                Node {
                    justify_content: JustifyContent::End,
                    ..default()
                },
                CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(text_field.char_count as f32 * CHARACTER_WIDTH)),
                CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(1.)),
                BackgroundColor(Color::WHITE),
            ))
            .with_child((
                Node::DEFAULT,
                Text::new("000"),
                TextLayout {
                    justify: JustifyText::Center,
                    linebreak: LineBreak::NoWrap
                },
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    ..default()
                },
                TextColor(Color::BLACK),
                CNodeSize::<NodeFontSize>::new(NodeSizeOperator::Em(1.)),
                text_field
            ))
            .id()
    }
}

// impl TSelectableUI for CTextField {
//     type InputData = Vec<char>;
//     type InputParam = EventReader<'static, 'static, KeyboardInput>;
//     type Node = Children;
//     type UiParams = Query<'static, 'static, &'static mut Text>;

//     fn get_input_data(input_resource: &mut Self::InputParam, prev_input_data: &Self::InputData) -> Option<Self::InputData> {
//         let mut has_new_inputs: Option<Self::InputData> = None;

//         for event in input_resource.read() {
//             if let ButtonState::Released = event.state { continue; }

//             match &event.logical_key {
//                 Key::Character(input) => {
//                     let mut input_chars = input.chars();
//                     if !input_chars.all(|c| c.is_ascii_digit()) { continue; }

//                     match &mut has_new_inputs {
//                         Some(has_new_inputs) => {
//                             has_new_inputs.extend(input_chars)
//                         },
//                         None => {
//                             let mut  new_inputs = prev_input_data.clone();
//                             new_inputs.extend(input_chars);
//                             has_new_inputs = Some(new_inputs);
//                         }
//                     }
//                 },
//                 _ => { }
//             }
//         }

//         return has_new_inputs;
//     }

//     fn update(&mut self, node: &mut Self::Node, ui_params: &mut Self::UiParams, input_data: &Self::InputData) {
//         for entity in node.iter() {
//             if let Ok(mut text) = ui_params.get_mut(*entity) {
//                 text.0 = String::from_iter(input_data);
//             }
//         }
//     }
// }