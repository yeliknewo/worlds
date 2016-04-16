use dorp::{
    IdManager, Id, IdType, Vec3, TickCount, Named, DorpErr
};

use core::{WEntity, WWorld, WMap};
use components::{Overseer};

pub fn new_overseer(manager: &mut IdManager, name: &'static str, zoom: Vec3, tick_count: TickCount, world: &mut WWorld) -> Result<WEntity, DorpErr> {
    let id = Id::new(manager, IdType::Entity);
    let overseer = Overseer::new(zoom, tick_count);
    let named = match Named::new(name, id, world) {
        Ok(named) => named,
        Err(err) => return Err(DorpErr::Dorp("Named New", Box::new(err))),
    };
    let wmap = WMap::new();
    Ok(
        WEntity::new(id)
        .with_named(named)
        .with_overseer(overseer)
        .with_wmap(wmap)
    )
}
