use bevy::{
    prelude::*,
    ecs::component::{StorageType, ComponentHooks}
};

use super::e_ui_selected::{EUISelected, EUIDeselected};


#[derive(Component)]
pub struct MSelectableUI;


pub struct MSelectedUI;

impl Component for MSelectedUI {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks
            .on_add(|mut world, entity, _component_id| {
                world.trigger_targets(EUISelected, entity);
            })
            .on_remove(|mut world, entity, _component_id| {
                world.trigger_targets(EUIDeselected, entity);
            });
    }
}


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
}


pub mod trigger {
    use super::*;
    

    pub fn update_outline_on_selected(
        trigger: Trigger<EUISelected>,
        mut outline_query: Query<&mut Outline>
    ) {

        let Ok(mut outline)  = outline_query.get_mut(trigger.entity())
            else { warn!("selected ui has no outline"); return; };
        outline.color = Color::srgb(0., 0., 1.);
    }

    pub fn update_outline_on_deselected(
        trigger: Trigger<EUIDeselected>,
        mut outline_query: Query<&mut Outline>
    ) {

        
        let Ok(mut outline) = outline_query.get_mut(trigger.entity())
            else { warn!("deselected ui has no outline"); return; };
        outline.color = Color::NONE;
    }
}