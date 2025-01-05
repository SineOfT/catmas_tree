use bevy::prelude::*;

use c_selectable_ui::{CSelectableUI, SelectableUIMarker};


pub mod grid;
pub mod digit_field;
pub mod e_selected_changed;
pub mod c_selectable_ui;
pub mod m_selected_ui;


#[derive(Bundle)]
pub struct SelectableUIBundle<M: SelectableUIMarker> {
    ui: CSelectableUI<M>,
    button: Button,
    outline: Outline
}

impl<M: SelectableUIMarker> SelectableUIBundle<M> {
    pub fn new(ui: CSelectableUI<M>, outline_width: Val) -> Self {
        Self {
            ui,
            button: Button,
            outline: Outline {
                width: outline_width,
                offset: Val::ZERO,
                color: Color::NONE
            }
        }
    }
}


fn implement_selectable_ui<M: SelectableUIMarker>(app: &mut App) {
    app
        .add_event::<e_selected_changed::ESelectedChanged<M>>()
        .add_systems(Update,
            (
                c_selectable_ui::update::update_on_change::<M>,
                m_selected_ui::update::update_selected_on_click::<M>
            )
        );
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
                (
                    m_selected_ui::update::update_outline_on_selected,
                    m_selected_ui::update::update_outline_on_deselected,

                    
                )
            );
    }
}