use bevy::prelude::*;

use m_selectable_ui::MSelectableUI;


pub mod grid;
pub mod digit_field;
pub mod m_selectable_ui;
pub mod e_ui_selected;


#[derive(Bundle)]
pub struct SelectableUIBundle {
    selectable: MSelectableUI,
    button: Button,
    outline: Outline
}

impl SelectableUIBundle {
    pub fn new(outline_width: Val) -> Self {
        Self {
            selectable: MSelectableUI,
            button: Button,
            outline: Outline {
                width: outline_width,
                offset: Val::ZERO,
                color: Color::NONE
            }
        }
    }
}


pub struct SelectableUIPlugin;

impl Plugin for SelectableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                grid::GridPlugin,
                digit_field::DigitFieldPlugin
            ))
            .add_systems(Update,
                m_selectable_ui::update::select_on_click
            )
            .add_observer(m_selectable_ui::trigger::update_outline_on_selected)
            .add_observer(m_selectable_ui::trigger::update_outline_on_deselected);
    }
}