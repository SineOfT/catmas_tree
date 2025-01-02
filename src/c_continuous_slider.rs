use bevy::prelude::*;

use super::{
    selectable_ui::{
        SelectableUIBundle,
        grid::{
            GridDimension,
            SelectableGrid,
            ArrowInputGrid
        },
        digit_field::FieldDigit,
        c_selectable_ui::CSelectableUI,
        e_selected_changed::ESelectedChanged
    },
    node_orientation::NodeOrientation,
    c_node_size::{CNodeSize, NodeSizeOperator, NodeJustify},
    digit_field::{
        e_digit_changed::EDigitChanged,
        c_digit_field::CDigitField
    }
};


#[derive(Component)]
pub struct CContinuousSlider {
    hover_slider_entity: Entity,
    fill_slider_entity: Entity,
    t_field_entity: Entity,
    min_value: f32,
    max_value: f32,
    t: f32
}

impl CContinuousSlider {    
    pub fn get_value(&self) -> f32 {
        f32::lerp(self.min_value, self.max_value, self.t)
    }


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
    ) -> Entity {

        let t_field_entity = CDigitField::build::<N>(
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
                ArrowInputGrid::new(1.),
                Self {
                    hover_slider_entity,
                    fill_slider_entity,
                    t_field_entity,
                    min_value: Self::get_value_of_digits(min_digits, decimal_position),
                    max_value: Self::get_value_of_digits(max_digits, decimal_position),
                    t: 0.
                }
            ))
            .add_child(hover_slider_entity)
            .add_child(fill_slider_entity)
            .id();

        commands
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
            .id()
    }
}


pub mod update {
    use super::*;


    pub fn update_slider_on_selected_changed(
        mut er_selected_changed: EventReader<ESelectedChanged<SelectableGrid<1>>>,
        mut ew_digit_changed: EventWriter<EDigitChanged>,
        mut slider_query: Query<&mut CContinuousSlider>,
        mut node_query: Query<&mut Node>,
    ) {

        for &ESelectedChanged { entity, data } in er_selected_changed.read() {
            let t = data[0];
            
            let Ok(mut slider) = slider_query.get_mut(entity)
                else { warn!("selected slider doesn't exist"); continue; };
                
            let Ok(mut fill_node) = node_query.get_mut(slider.fill_slider_entity)
                else { warn!("slider's fill slider doesn't exist"); continue; };
            
            slider.t = t;
            NodeOrientation::set_justify(&mut fill_node, Val::Percent(t * 100.));

            ew_digit_changed.send(EDigitChanged(slider.get_value()));
        }
    }
}