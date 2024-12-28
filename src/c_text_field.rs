use bevy::prelude::*;

use super::{
    em::{CEmWidth, CEmHeight, CEmFontSize},
    c_selectable_ui::CSelectableUI
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
                CEmWidth(text_field.char_count as f32 * CHARACTER_WIDTH),
                CEmHeight(1.),
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
                CEmFontSize(1.),
                CSelectableUI::new(),
                text_field
            ))
            .id()
    }
}