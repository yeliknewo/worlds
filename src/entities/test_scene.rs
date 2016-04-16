use dorp::{
    IdManager, Id, IdType, Vec4, Entity, DorpErr, Vec3
};

use core::{WEntity, WScene, WCoords, OVERSEER_NAME};
use entities::{new_province, new_chunk, new_chunk_renderable, new_base, new_overseer};

pub fn new_test_scene(manager: &mut IdManager) -> WEntity {
    let id = Id::new(manager, IdType::Entity);
    let scene = WScene::new(Box::new(|manager, world, sync_data| {
        let zoom = Vec3::from([0.1, 0.1, 0.1]);
        let overseer_id = {
            let overseer = match new_overseer(manager, OVERSEER_NAME, zoom, 0, world) {
                Ok(overseer) => overseer,
                Err(err) => return Err(DorpErr::Dorp("New Overseer", Box::new(err))),
            };
            let overseer_id = overseer.get_id();
            world.add_entity(overseer);
            overseer_id
        };
        let province_id = {
            let province = new_province(Vec4::from([0.0, 1.0, 0.0, 1.0]), manager, sync_data);
            let id = province.get_id();
            world.add_entity(province);
            id
        };
        let province2_id = {
            let province = new_province(Vec4::from([1.0, 0.0, 0.0, 1.0]), manager, sync_data);
            let id = province.get_id();
            world.add_entity(province);
            id
        };
        {
            let base = new_base(manager, world);
            let chunk_renderable = match new_chunk_renderable(manager, &base) {
                Ok(renderable) => renderable,
                Err(err) => return Err(DorpErr::Dorp("New Chunk renderable manager base", Box::new(err))),
            };
            for y in -5..6 {
                for x in -5..6 {
                    let coords = WCoords::new(x,y);
                    if x < 0 {
                        let chunk = match new_chunk(manager, &chunk_renderable, zoom, &coords, match world.get_mut_entity_by_id(province2_id) {
                            Some(province) => province,
                            None => return Err(DorpErr::Base("World Get mut Entity by Id Province id was none")),
                        }) {
                            Ok(chunk) => chunk,
                            Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                        };
                        world.add_entity(chunk);
                    } else {
                        let chunk = match new_chunk(manager, &chunk_renderable, zoom, &coords, match world.get_mut_entity_by_id(province_id) {
                            Some(province) => province,
                            None => return Err(DorpErr::Base("World Get mut Entity by Id Province id was none")),
                        }) {
                            Ok(chunk) => chunk,
                            Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                        };
                        world.add_entity(chunk);
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
