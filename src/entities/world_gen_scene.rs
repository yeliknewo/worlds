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
        let base = new_base(manager, world);
        let chunk_renderable = match new_chunk_renderable(manager, &base) {
            Ok(renderable) => renderable,
            Err(err) => return Err(DorpErr::Dorp("new chunk renderable", Box::new(err))),
        };
        let mut coords = WCoords::new(0, 0);
        let province_count = 2;
        let province_size = 3;
        for _ in 0..province_count {
            let province_id = {
                let province = new_province(Vec4::from([rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 1.0]), manager, sync_data);
                let id = province.get_id();
                world.add_entity(province);
                id
            };
            for _ in 0..province_size {
                let chunk_entity = match new_chunk(manager, &chunk_renderable, zoom, &coords, province_id, world) {
                    Ok(chunk) => chunk,
                    Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                };
                world.add_entity(chunk_entity);
                match world.take_entity_by_id(province_id) {
                    Some(province_entity) => {
                        match province_entity.get_province() {
                            Some(province) => {
                                let chunks = province.get_chunks();
                                for chunk_id in chunks.iter() {
                                    match world.take_entity_by_id(*chunk_id) {
                                        Some(mut chunk_entity) => {
                                            match chunk_entity.get_mut_chunk() {
                                                Some(chunk) => {
                                                    match chunk.prep_update_neighbors(match world.get_entity_by_id(overseer_id) {
                                                        Some(overseer_entity) => match overseer_entity.get_wmap() {
                                                            Some(wmap) => wmap,
                                                            None => return Err(DorpErr::Base("Overseer entity get wmap was none")),
                                                        },
                                                        None => return Err(DorpErr::Base("world get entity by id overseer id was none")),
                                                    }) {
                                                        Ok(()) => (),
                                                        Err(err) => return Err(DorpErr::Dorp("Chunk prep update neighbors", Box::new(err))),
                                                    };
                                                    match chunk.update_neighbors() {
                                                        Ok(()) => (),
                                                        Err(err) => return Err(DorpErr::Dorp("Chunk update neighbors", Box::new(err))),
                                                    }
                                                },
                                                None => return Err(DorpErr::Base("Chunk entity get mut chunk was none")),
                                            }
                                            world.add_entity(chunk_entity);
                                        },
                                        None => return Err(DorpErr::Base("world take entity by id chunk id was none")),
                                    }
                                }
                                let empty = match province.get_empty_neighbors(world) {
                                    Ok(empty) => empty,
                                    Err(err) => return Err(DorpErr::Dorp("province get empty neighbors world", Box::new(err))),
                                };
                                let length = empty.len();
                                if length == 0 {
                                    return Err(DorpErr::Base("Length was zero"));
                                }
                                let index = rng.gen_range(0, length);
                                coords = match empty.get(index) {
                                    Some(coords) => WCoords::new(coords.get_x(), coords.get_y()),
                                    None => return Err(DorpErr::Base("Empty get index was none")),
                                };
                            }
                            None => return Err(DorpErr::Base("Province entity get province was none")),
                        }
                        world.add_entity(province_entity);
                    },
                    None => return Err(DorpErr::Base("World Take Entity By Id Province Id was none")),
                }
            }
        }
        println!("World Gen Scene Loaded");
        Ok(())
    }));
    WEntity::new(id)
    .with_scene(scene)
}
