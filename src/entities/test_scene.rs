use std::sync::{Arc};

use dorp::{
    IdManager, Id, IdType, Named, Vec4, Entity, DorpErr, OptErr
};

use core::{WEntity, WScene, WMap, WCoords, WMAP_NAME};
use entities::{new_province, new_chunk, new_chunk_renderable, new_base};

pub fn new_test_scene(manager: &mut IdManager) -> WEntity {
    let id = Id::new(manager, IdType::Entity);
    let scene = WScene::new(Box::new(|manager, world, sync_data| {
        let wmap_id = {
            let id = Id::new(manager, IdType::Entity);
            let wmap = WMap::new();
            let named = match Named::new(WMAP_NAME, id, world) {
                Ok(named) => named,
                Err(err) => return Err(DorpErr::Dorp("Named New WMAP_NAME Id World", Box::new(err))),
            };
            if let Err(err) = world.add_entity(WEntity::new(id)
                .with_wmap(wmap)
                .with_named(named)
            ) {
                return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
            }
        };
        let province_id = {
            let province = new_province(Vec4::from([0.0, 1.0, 0.0, 1.0]), manager, sync_data);
            let id = province.get_id();
            if let Err(err) = world.add_entity(province) {
                return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
            }
            id
        };
        let province2_id = {
            let province = new_province(Vec4::from([1.0, 0.0, 0.0, 1.0]), manager, sync_data);
            let id = province.get_id();
            if let Err(err) = world.add_entity(province) {
                return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
            }
            id
        };
        {
            let base = Arc::new(new_base(manager, world));
            let chunk_renderable = Arc::new(match new_chunk_renderable(manager, base) {
                Ok(renderable) => renderable,
                Err(err) => return Err(DorpErr::Dorp("New Chunk renderable manager base", Box::new(err))),
            });
            for y in -5..6 {
                for x in -5..6 {
                    let coords = Arc::new(WCoords::new(x,y));
                    if x < 0 {
                        let chunk = match new_chunk(manager, chunk_renderable.clone(), coords, match world.get_mut_entity_by_id(province2_id) {
                            OptErr::Full(province) => province,
                            OptErr::Empty => return Err(DorpErr::Base("World Get mut Entity by Id Province id was none")),
                            OptErr::Error(err) => return Err(DorpErr::Dorp("World Get mut Entity by Id Province id", Box::new(err))),
                        }, sync_data) {
                            Ok(chunk) => chunk,
                            Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                        };
                        if let Err(err) = world.add_entity(chunk) {
                            return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
                        }
                    } else {
                        let chunk = match new_chunk(manager, chunk_renderable.clone(), coords, match world.get_mut_entity_by_id(province_id) {
                            OptErr::Full(province) => province,
                            OptErr::Empty => return Err(DorpErr::Base("World Get mut Entity by Id Province id was none")),
                            OptErr::Error(err) => return Err(DorpErr::Dorp("World Get mut Entity by Id Province id", Box::new(err))),
                        }, sync_data) {
                            Ok(chunk) => chunk,
                            Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                        };
                        if let Err(err) = world.add_entity(chunk) {
                            return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
                        }
                    }
                }
            }
        }
        println!("Test Scene Loaded");
        Ok(())
    }));
    WEntity::new(id)
    .with_scene(scene)
}
