use bevy::prelude::*;

use super::{
    m_selectable_ui::MSelectedUI,
};


pub trait SelectableUIMarker: Sync + Send + 'static {
    type ConstructorParam: Sync + Send;
    type Data: Sync + Send + Clone;
    const DEFAULT_DATA: Self::Data;
}


#[derive(Component)]
pub struct CSelectableUI<M: SelectableUIMarker> {
    param: M::ConstructorParam,
    data: M::Data
}

impl<M: SelectableUIMarker> CSelectableUI<M> {
    pub fn new(param: M::ConstructorParam) -> Self {
        Self { param, data: M::DEFAULT_DATA }
    }

    pub fn new_with_data(param: M::ConstructorParam, data: M::Data) -> Self {
        Self { param, data }
    }

    pub fn param(&self) -> &M::ConstructorParam { &self.param }
    pub fn data(&self) -> &M::Data { &self.data }
    pub fn data_mut(&mut self) -> &mut M::Data { &mut self.data }
    pub fn set_data(&mut self, data: M::Data) { self.data = data; }
}


// pub mod update {
//     use super::*;


//     pub fn update_on_change<M: SelectableUIMarker>(
//         mut ew_selected_changed: EventWriter<ESelectedChanged<M>>,
//         selected_ui_query: Query<(Entity, &CSelectableUI<M>), (With<MSelectedUI>, Changed<CSelectableUI<M>>)>
//     ) {

//         for (entity, ui) in selected_ui_query.iter() {
//             ew_selected_changed.send(ESelectedChanged { entity, data: ui.data.clone() });
//         }
//     }
// }