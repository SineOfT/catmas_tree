use bevy::prelude::*;

use super::c_continuous_slider_entities::CContinuousSliderEntities;

use super::super::{
    selectable_ui::grid::{EGridChanged, EGridUpdated},
    c_node_size::{CNodeSize, NodeAlign},
    s_node_orientation::NodeOrientation
};


#[derive(Component)]
pub struct CContinuousSliderData {
    t: f32
}

impl CContinuousSliderData {
    fn new() -> Self {
        Self { t: 0. }
    }

    pub fn build<const N: usize, const D: usize>(
        commands: &mut Commands,
        orientation: NodeOrientation,
        length: CNodeSize<NodeAlign>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> Entity {

        let entity = CContinuousSliderEntities::build(
            commands,
            orientation,
            length,
            empty_color,
            hover_color,
            fill_color,
            ()
        );

        commands.entity(entity)
            .insert(Self::new())
            .observe(trigger::update_slider);

        entity
    }
}


mod trigger {
    use super::*;


    fn t_to_value(range: Vec2, t: f32) -> f32 {
        f32::lerp(range.x, range.y, t)
    }

    fn value_to_t(range: Vec2, value: f32) -> f32 {
        f32::inverse_lerp(range.x, range.y, value)
    }


    pub fn update_slider(
        mut trigger: Trigger<EGridChanged<{CContinuousSliderEntities::DIMENSION_COUNT}, ()>>,
        mut commands: Commands,
        mut continuous_slider_data_query: Query<&mut CContinuousSliderData>
    ) {

        trigger.propagate(false);

        let entity = trigger.entity();
        let Ok(mut data) = continuous_slider_data_query.get_mut(entity)
            else { warn!("changed slider doesn't have data parent"); return; };
        
        let t = trigger.event().t[0];
        data.t = t;

        commands.trigger_targets(EGridUpdated { t: [t] }, entity);
    }
}