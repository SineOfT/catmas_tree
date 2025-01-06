use bevy::prelude::*;

use super::c_selectable_ui::{CSelectableUI, SelectableUIMarker};


#[derive(Component)]
pub struct MSelectableUI;


#[derive(Component)]
pub struct MSelectedUI;


pub mod update {
    use super::*;


    pub fn select_on_click(
        mut commands: Commands,
        selectable_ui_query: Query<(Entity, &Interaction), (With<MSelectableUI>, Without<MSelectedUI>, Changed<Interaction>)>,
        selected_ui_query: Query<Entity, With<MSelectedUI>>
    ) {

        for (entity, interaction) in selectable_ui_query.iter() {
            match *interaction {
                Interaction::Pressed => {
                    for selected_entity in selected_ui_query.iter() {
                        commands.entity(selected_entity).remove::<MSelectedUI>();
                    }
                    commands.entity(entity).insert(MSelectedUI);
                },
                Interaction::Hovered => { },
                Interaction::None => { }
            }
        }
    }

    pub fn update_outline_on_selected(
        mut selected_node_query: Query<&mut Outline, Added<MSelectedUI>>
    ) {

        for mut outline in selected_node_query.iter_mut() {
            outline.color = Color::srgb(0., 0., 1.);
        }
    }

    pub fn update_outline_on_deselected(
        mut deselected: RemovedComponents<MSelectedUI>,
        mut node_query: Query<&mut Outline>
    ) {

        for entity in deselected.read() {
            let Ok(mut outline) = node_query.get_mut(entity)
                else { warn!("deselected entity doesn't exist"); continue; };
            outline.color = Color::NONE;
        }
    }
}