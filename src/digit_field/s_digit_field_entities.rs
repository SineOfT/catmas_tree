use bevy::prelude::*;

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeFontSize},
    selectable_ui::{
        SelectableUIBundle,
        c_selectable_ui::CSelectableUI,
        digit_field::{ECursorUpdated, EDigitsUpdated, SelectableDigitField, DigitFieldParam, CKeyInputField}
    },
    s_field_digit::FieldDigit
};


const FONT_PATH: &'static str = "source_code_pro.ttf";
const CHARACTER_WIDTH: f32 = 0.55;


pub struct DigitFieldEntities {
    field_entity: Entity,
    cursor_entity: Entity
}

impl DigitFieldEntities {
    pub fn field_entity(&self) -> Entity { self.field_entity }
    pub fn cursor_entity(&self) -> Entity { self.cursor_entity }

    pub fn build<const N: usize>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        min_digits: [FieldDigit; N],
        max_digits: [FieldDigit; N],
        mut decimal_position: Option<usize>
    ) -> (Entity, Self) {

        if let Some(position) = decimal_position {
            if position >= N-1 {
                warn!("decimal position is not in valid space");
                decimal_position = None;
            }
        }

        let field_entity = commands
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
            .observe(trigger::update_digits_text::<N>)
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
            .observe(trigger::update_cursor_node::<N>)
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
                CKeyInputField
            ));
        
        entity_commands.add_child(field_entity);
        if let Some(decimal_entity) = decimal_entity { entity_commands.add_child(decimal_entity); }
        entity_commands.add_child(cursor_entity);
        
        (entity_commands.id(), Self { field_entity, cursor_entity })
    }
}


mod trigger {
    use super::*;


    fn digits_to_string<const N: usize>(digits: [FieldDigit; N]) -> String {
        String::from_iter(digits.iter().map(|digit| digit.as_char()))
    }

    pub fn update_digits_text<const N: usize>(
        trigger: Trigger<EDigitsUpdated<N>>,
        mut text_query: Query<&mut Text>
    ) {

        let Ok(mut text) = text_query.get_mut(trigger.entity())
            else { warn!("updated digit has no text"); return; };

        let digits = trigger.event().0;
        let digits_string = digits_to_string(digits);

        text.0 = digits_string;
    }


    pub fn update_cursor_node<const N: usize>(
        trigger: Trigger<ECursorUpdated>,
        mut node_query: Query<&mut Node>
    ) {

        let Ok(mut node) = node_query.get_mut(trigger.entity())
            else { warn!("updated cursor has no node"); return; };

        let cursor = trigger.event().0;
        
        node.left = Val::Percent(100. / (N as f32) * (cursor as f32));
    }
}