use bevy::prelude::*;

use super::super::{
    c_node_size::{CNodeSize, NodeSizeOperator, NodeWidth, NodeHeight, NodeFontSize},
    selectable_ui::{
        SelectableUIBundle,
        digit_field::{ECursorUpdated, EDigitsUpdated, CDigitField, CKeyInputField}
    },
    s_field_digit::FieldDigits
};


const FONT_PATH: &'static str = "source_code_pro.ttf";
const CHARACTER_WIDTH: f32 = 0.55;


#[derive(Component)]
pub struct CDigitFieldEntities {
    digits_entity: Entity,
    cursor_entity: Entity
}

impl CDigitFieldEntities {
    pub fn build<const N: usize, const D: usize, T: Sync + Send + Copy + 'static>(
        commands: &mut Commands,
        asset_server: &AssetServer,
        min_digits: FieldDigits<N, D>,
        max_digits: FieldDigits<N, D>,
        tag: T
    ) -> Entity {

        let digits_entity = commands
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

        let decimal_entity = if D < N-1 {
            Some(
                commands
                .spawn((
                    Node {
                        left: Val::Percent(100. / (N as f32) * (D as f32 + 0.5)),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    CNodeSize::<NodeWidth>::new(NodeSizeOperator::Em(0.1)),
                    CNodeSize::<NodeHeight>::new(NodeSizeOperator::Em(0.1))
                ))
                .id()
            )
        } else {
            None
        };

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
                SelectableUIBundle::new(Val::Px(5.)),
                CDigitField::new(min_digits, max_digits, tag),
                CKeyInputField,
                Self { digits_entity, cursor_entity }
            ));
        
        entity_commands.add_child(digits_entity);
        if let Some(decimal_entity) = decimal_entity { entity_commands.add_child(decimal_entity); }
        entity_commands.add_child(cursor_entity);

        entity_commands
            .observe(trigger::update_digits_text::<N, D>)
            .observe(trigger::update_cursor_node::<N>);
        
        entity_commands.id()
    }


    pub fn get_size_em<const N: usize, const D: usize>() -> Vec2 {
        Vec2::new(CHARACTER_WIDTH * N as f32, 1.)
    }
}


mod trigger {
    use super::*;


    fn digits_to_string<const N: usize, const D: usize>(digits: &FieldDigits<N, D>) -> String {
        String::from_iter(digits.iter().map(|digit| digit.as_char()))
    }

    pub fn update_digits_text<const N: usize, const D: usize>(
        trigger: Trigger<EDigitsUpdated<N, D>>,
        digit_field_entities_query: Query<&CDigitFieldEntities>,
        mut text_query: Query<&mut Text>
    ) {

        let Ok(digit_field_entities) = digit_field_entities_query.get(trigger.entity())
            else { warn!("updated digits has no entities"); return; };

        let Ok(mut text) = text_query.get_mut(digit_field_entities.digits_entity)
            else { warn!("updated digit has no text"); return; };

        let digits = &trigger.event().digits;
        let digits_string = digits_to_string(digits);

        text.0 = digits_string;
    }


    pub fn update_cursor_node<const N: usize>(
        trigger: Trigger<ECursorUpdated>,
        digit_field_entities_query: Query<&CDigitFieldEntities>,
        mut node_query: Query<&mut Node>
    ) {

        let Ok(digit_field_entities) = digit_field_entities_query.get(trigger.entity())
        else { warn!("updated digits has no entities"); return; };

        let Ok(mut node) = node_query.get_mut(digit_field_entities.cursor_entity)
            else { warn!("updated cursor has no node"); return; };

        let cursor = trigger.event().cursor;
        
        node.left = Val::Percent(100. / (N as f32) * (cursor as f32));
    }
}