use bevy::prelude::*;

use super::{
    c_slider_data::CSliderData
};

use super::super::{
    selectable_ui::{
        SelectableUIBundle,
        grid::{
            GridDimension,
            SelectableGrid,
            CArrowInputGrid
        },
        c_selectable_ui::CSelectableUI,
        e_selected_changed::ESelectedChanged
    },
    node_orientation::NodeOrientation,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeJustify},
    digit_field::{
        e_digit_changed::EDigitChanged,
        s_digit_field_entities::DigitFieldEntities
    },
    s_field_digit::FieldDigit
};


pub struct ContinuousSliderEntities {
    hover_slider_entity: Entity,
    fill_slider_entity: Entity,
    t_field_entities: DigitFieldEntities
}

impl ContinuousSliderEntities {

    pub fn hover_slider_entity(&self) -> Entity { self.hover_slider_entity }
    pub fn fill_slider_entity(&self) -> Entity { self.fill_slider_entity }
    pub fn t_field_entities(&self) -> &DigitFieldEntities { &self.t_field_entities }

    fn get_value_of_digits<const N: usize>(digits: [FieldDigit; N], decimal_position: Option<usize>) -> f32 {
        let decimal_position = decimal_position.unwrap_or(N-1) as i32;
        let mut value = 0.;
        for (i, digit) in digits.iter().enumerate() {
            let digit_position = decimal_position - i as i32;
            value += digit.as_num::<f32>() * 10_f32.powi(digit_position);
        }
        value
    }

    pub fn build<const N: usize>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        orientation: NodeOrientation,
        justify_size: CNodeSize<NodeJustify>,
        min_digits: [FieldDigit; N],
        max_digits: [FieldDigit; N],
        decimal_position: Option<usize>,
        empty_color: Color,
        hover_color: Color,
        fill_color: Color
    ) -> (Entity, Self) {

        let (t_field_entity, t_field_entities) = DigitFieldEntities::build::<N>(
            commands,
            asset_server,
            min_digits,
            max_digits,
            decimal_position
        );

        let hover_slider_entity = commands
            .spawn((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(hover_color),
                ZIndex(0)
            ))
            .id();

        let fill_slider_entity = commands
            .spawn((
                orientation.apply(Node {
                    position_type: PositionType::Absolute,
                    width: Val::ZERO,
                    height: Val::Percent(100.),
                    ..default()
                }),
                BackgroundColor(fill_color),
                ZIndex(1)
            ))
            .id();

        let slider_entity = commands
            .spawn((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    flex_basis: Val::Percent(100.),
                    ..default()
                }),
                CNodeSize::<NodeJustify>::new(NodeSizeOperator::Em(0.5)),
                BackgroundColor(empty_color),
                SelectableUIBundle::new(
                    CSelectableUI::<SelectableGrid<1>>::new(orientation.apply_to_grid_dimensions([GridDimension::X])),
                    Val::Px(5.),
                ),
                CArrowInputGrid::new(1.),
            ))
            .add_child(hover_slider_entity)
            .add_child(fill_slider_entity)
            .id();
 
        let entity = commands
            .spawn((
                orientation.apply(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Stretch,
                    align_items: AlignItems::Center,
                    ..default()
                }),
                justify_size
            ))
            .add_child(slider_entity)
            .add_child(t_field_entity)
            .id();

        (entity, Self { hover_slider_entity, fill_slider_entity, t_field_entities })
    }

//     pub fn build_complete<const N: usize>(
//         commands: &mut Commands,
//         asset_server: &AssetServer,
//         orientation: NodeOrientation,
//         justify_size: CNodeSize<NodeJustify>,
//         min_digits: [FieldDigit; N],
//         max_digits: [FieldDigit; N],
//         decimal_position: Option<usize>,
//         empty_color: Color,
//         hover_color: Color,
//         fill_color: Color,
//     ) -> Entity {

//         let (entity, slider_entities) = Self::build::<N>(
//             commands,
//             asset_server,
//             orientation,
//             justify_size,
//             min_digits,
//             max_digits,
//             decimal_position,
//             empty_color,
//             hover_color,
//             fill_color
//         );

//         commands.entity(entity).insert(slider_entities);

//         entity
//     }
}


pub mod update {
    use super::*;


    // pub fn update_slider_on_selected_changed(
    //     mut er_selected_changed: EventReader<ESelectedChanged<SelectableGrid<1>>>,
    //     mut ew_digit_changed: EventWriter<EDigitChanged>,
    //     mut slider_query: Query<&mut CContinuousSlider>,
    //     mut node_query: Query<&mut Node>,
    // ) {

    //     for &ESelectedChanged { entity, data } in er_selected_changed.read() {
    //         let t = data[0];
            
    //         let Ok(mut slider) = slider_query.get_mut(entity)
    //             else { warn!("selected slider doesn't exist"); continue; };
                
    //         let Ok(mut fill_node) = node_query.get_mut(slider.fill_slider_entity)
    //             else { warn!("slider's fill slider doesn't exist"); continue; };
            
    //         slider.t = t;
    //         NodeOrientation::set_justify(&mut fill_node, Val::Percent(t * 100.));

    //         ew_digit_changed.send(EDigitChanged(slider.get_value()));
    //     }
    // }
}