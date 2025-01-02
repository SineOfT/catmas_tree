use bevy::prelude::*;

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeFontSize},
    selectable_ui::{
        SelectableUIBundle,
        e_selected_changed::ESelectedChanged,
        c_selectable_ui::{CSelectableUI, SelectableUIMarker},
        digit_field::{SelectableDigitField, DigitFieldParam, DigitFieldData, FieldDigit, KeyInputField}
    }
};


const FONT_PATH: &'static str = "source_code_pro.ttf";
const CHARACTER_WIDTH: f32 = 0.55;


#[derive(Component)]
pub struct CDigitField {
    text_entity: Entity,
    cursor_entity: Entity,
    char_count: u8,
    decimal_position: Option<usize>,
    value: f32
}

impl CDigitField {
    pub fn build<const N: usize>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        min_digits: [FieldDigit; N],
        max_digits: [FieldDigit; N],
        mut decimal_position: Option<usize>
    ) -> Entity {

        if let Some(position) = decimal_position {
            if position >= N-1 {
                warn!("decimal position is not in valid space");
                decimal_position = None;
            }
        }

        let text_entity = commands
            .spawn((
                Node::default(),
                Text::new("0".repeat(N)),
                TextLayout {
                    justify: JustifyText::Center,
                    linebreak: LineBreak::NoWrap
                },
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    ..default()
                },
                TextColor(Color::BLACK),
                CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(0.8)),
                CNodeSize::<NodeFontSize>::new(NodeSizeOperator::Em(0.8))
            ))
            .id();

        let decimal_entity = decimal_position.map(|decimal_position| {
            commands
                .spawn((
                    Node {
                        left: Val::Percent(100. / (N as f32) * (decimal_position as f32 + 0.5)),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(0.1)),
                    CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(0.1))
                ))
                .id()
        });

        let cursor_entity = commands
            .spawn((
                Node {
                    width: Val::Percent(100. / (N as f32)),
                    left: Val::Percent(0.),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(0.1))
            ))
            .id();

        let mut entity_commands = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(N as f32 * CHARACTER_WIDTH)),
                CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(1.)),
                BackgroundColor(Color::WHITE),
                SelectableUIBundle::new(
                    CSelectableUI::<SelectableDigitField<N>>::new(DigitFieldParam::new(
                        min_digits,
                        max_digits,
                        decimal_position
                    )),
                    Val::Px(5.),
                ),
                KeyInputField,
                CDigitField { text_entity, cursor_entity, char_count: N as u8, decimal_position, value: 0. }
            ));
        
        entity_commands.add_child(text_entity);
        if let Some(decimal_entity) = decimal_entity { entity_commands.add_child(decimal_entity); }
        entity_commands.add_child(cursor_entity);
        
        entity_commands.id()
    }
}


pub mod update {
    use super::*;


    pub fn update_text_on_selected_changed<const N: usize>(
        mut er_selected_changed: EventReader<ESelectedChanged<SelectableDigitField<N>>>,
        mut digit_field_query: Query<&mut CDigitField>,
        mut text_query: Query<&mut Text>,
        mut node_query: Query<&mut Node>
    ) {

        for ESelectedChanged { entity, data } in er_selected_changed.read() {
            let Ok(mut digit_field) = digit_field_query.get_mut(*entity)
                else { warn!("selected digit field doesn't exist"); continue; };
            let Ok(mut text) = text_query.get_mut(digit_field.text_entity)
                else { warn!("digit field's text doesn't exist"); continue; };
            let Ok(mut cursor_node) = node_query.get_mut(digit_field.cursor_entity)
                else { warn!("digit field's cursor doesn't exist"); continue; };

            let digits_string = data.digits_string();
            let Ok(digits_value) = digits_string.parse()
                else { warn!("digits string {} is not a valid float", digits_string); continue; };
            
            let cursor = data.cursor();
            info!(digits_string);
            info!(cursor);
            
            digit_field.value = digits_value;
            text.0 = digits_string;
            cursor_node.left = Val::Percent(100. / (N as f32) * (cursor as f32));
        }
    }
}