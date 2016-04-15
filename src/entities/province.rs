use dorp::{
    IdManager, Vec4, Id, IdType, SyncData
};

use core::{WEntity};
use components::{Province};

pub fn new_province(color: Vec4, manager: &mut IdManager, sync_data: &mut SyncData) -> WEntity {
    let id = Id::new(manager, IdType::Entity);
    let province = Province::new_with_color(manager, sync_data, color);
    WEntity::new(id)
    .with_province(province)
}
