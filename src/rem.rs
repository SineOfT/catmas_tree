use bevy::prelude::*;



#[derive(Component)]
pub struct CRemWidth(pub f32);
impl CRemWidth { fn to_val(&self, rem: &RRem) -> Val { Val::Px(self.0 * rem.0) } }

#[derive(Component)]
pub struct CRemHeight(pub f32);
impl CRemHeight { fn to_val(&self, rem: &RRem) -> Val { Val::Px(self.0 * rem.0) } }

#[derive(Component)]
pub struct CRemFontSize(pub f32);
impl CRemFontSize { fn to_val(&self, rem: &RRem) -> f32 { self.0 * rem.0 } }


#[derive(Resource)]
pub struct RRem(pub f32);



pub mod update {
    use super::*;


    pub fn update_widths(
        rem: Res<RRem>,
        mut node_query: Query<(&mut Node, &CRemWidth)>
    ) {

        for (mut node, width) in node_query.iter_mut() {
            node.width = width.to_val(&rem);
        }
    }

    pub fn update_heights(
        rem: Res<RRem>,
        mut node_query: Query<(&mut Node, &CRemHeight)>
    ) {
        
        for (mut node, height) in node_query.iter_mut() {
            node.height = height.to_val(&rem);
        }
    }

    pub fn update_font_sizes(
        rem: Res<RRem>,
        mut text_font_query: Query<(&mut TextFont, &CRemFontSize)>
    ) {

        for (mut text_font, font_size) in text_font_query.iter_mut() {
            text_font.font_size = font_size.to_val(&rem);
        }
    }
}