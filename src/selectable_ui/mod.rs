use bevy::prelude::*;

use c_selectable_ui::{CSelectableUI, SelectableUIMarker};
use grid::SelectableGrid;
use digit_field::SelectableDigitField;


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


pub struct SelectableUIPlugin;

impl Plugin for SelectableUIPlugin {
    fn build(&self, app: &mut App) {
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
        
        app
            .add_systems(Update,
                (
                    m_selected_ui::update::update_outline_on_selected,
                    m_selected_ui::update::update_outline_on_deselected,

                    grid::update::update_selected_arrow_input_grid::<1>,
                    grid::update::update_selected_arrow_input_grid::<2>,
                    grid::update::update_selected_arrow_input_grid::<3>,
                    grid::update::update_selected_arrow_input_grid::<4>,

                    digit_field::update::update_selected_key_input_field::<1>,
                    digit_field::update::update_selected_key_input_field::<2>,
                    digit_field::update::update_selected_key_input_field::<3>,
                    digit_field::update::update_selected_key_input_field::<4>,
                    digit_field::update::update_selected_key_input_field::<5>,
                    digit_field::update::update_selected_key_input_field::<6>
                )
            );

        
            implement_selectable_ui::<SelectableGrid<1>>(app);
            implement_selectable_ui::<SelectableGrid<2>>(app);
            implement_selectable_ui::<SelectableGrid<3>>(app);
            implement_selectable_ui::<SelectableGrid<4>>(app);
            implement_selectable_ui::<SelectableDigitField<1>>(app);
            implement_selectable_ui::<SelectableDigitField<2>>(app);
            implement_selectable_ui::<SelectableDigitField<3>>(app);
            implement_selectable_ui::<SelectableDigitField<4>>(app);
            implement_selectable_ui::<SelectableDigitField<5>>(app);
            implement_selectable_ui::<SelectableDigitField<6>>(app);
    }
}