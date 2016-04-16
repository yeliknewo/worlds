use std::sync::{Arc};

use rand::{Rng, thread_rng};

use dorp::{
    IdManager, Id, IdType, Vec3, DorpErr, Entity, Vec4
};

use core::{WEntity, WScene, OVERSEER_NAME, WCoords};
use entities::{new_overseer, new_province, new_base, new_chunk, new_chunk_renderable};

pub fn new_world_gen_scene(manager: &mut IdManager) -> WEntity {
    let id = Id::new(manager, IdType::Entity);
    let scene = WScene::new(Box::new(|manager, world, sync_data| {
        let mut rng = thread_rng();
        let zoom = Vec3::from([1.0, 1.0, 1.0]);
        let overseer_id = {
            let overseer = match new_overseer(manager, OVERSEER_NAME, zoom, 0, world) {
                Ok(overseer) => overseer,
                Err(err) => return Err(DorpErr::Dorp("New Overseer", Box::new(err))),
            };
            let overseer_id = overseer.get_id();
            if let Err(err) = world.add_entity(overseer) {
                return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
            }
            overseer_id
        };
        let base = Arc::new(new_base(manager, world));
        let chunk_renderable = Arc::new(match new_chunk_renderable(manager, base) {
            Ok(renderable) => renderable,
            Err(err) => return Err(DorpErr::Dorp("new chunk renderable", Box::new(err))),
        });
        let mut coords = Arc::new(WCoords::new(0, 0));
        let province_count = 1;
        for _ in 0..province_count {
            let mut province = new_province(Vec4::from([rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0]), manager, sync_data);
            {
                let overseer_entity = match world.get_entity_by_id(overseer_id) {
                    Some(overseer_entity) => overseer_entity,
                    None => return Err(DorpErr::Base("World Get Entity by id overseer id was none")),
                };
                let wmap = match overseer_entity.get_wmap() {
                    Some(wmap) => wmap,
                    None => return Err(DorpErr::Base("Overseer entity get wmap was none")),
                };
                while wmap.get(coords.get_x(), coords.get_y()).is_some() {
                    coords = match rng.gen_range::<u8>(0, 4) {
                        0 => Arc::new(WCoords::new(coords.get_x() + 1, coords.get_y())),
                        1 => Arc::new(WCoords::new(coords.get_x() - 1, coords.get_y())),
                        2 => Arc::new(WCoords::new(coords.get_x(), coords.get_y() + 1)),
                        3 => Arc::new(WCoords::new(coords.get_x(), coords.get_y() - 1)),
                        _ => return Err(DorpErr::Base("Problem with rng")),
                    }
                }
            }
            let chunk = match new_chunk(manager, chunk_renderable.clone(), zoom, coords.clone(), &mut province) {
                Ok(chunk) => chunk,
                Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
            };
            if let Err(err) = world.add_entity(chunk) {
                return Err(DorpErr::Dorp("World Add Entity", Box::new(err)));
            }
        }
        println!("World Gen Scene Loaded");
        Ok(())
    }));
    WEntity::new(id)
    .with_scene(scene)
}
