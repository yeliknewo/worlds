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
            world.add_entity(overseer);
            overseer_id
        };
        let base = new_base(manager, world);
        let chunk_renderable = match new_chunk_renderable(manager, &base) {
            Ok(renderable) => renderable,
            Err(err) => return Err(DorpErr::Dorp("new chunk renderable", Box::new(err))),
        };
        let mut coords = WCoords::new(0, 0);
        let province_count = 1;
        let province_size = 3;
        for _ in 0..province_count {
            let mut province = new_province(Vec4::from([rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), rng.gen_range::<f32>(0.0, 1.0), 1.0]), manager, sync_data);
            for _ in 0..province_size {
                let chunk_entity = match new_chunk(manager, &chunk_renderable, zoom, &coords, &mut province, world) {
                    Ok(chunk) => chunk,
                    Err(err) => return Err(DorpErr::Dorp("New Chunk", Box::new(err))),
                };
                world.add_entity(chunk_entity);
            }

        }
        println!("World Gen Scene Loaded");
        Ok(())
    }));
    WEntity::new(id)
    .with_scene(scene)
}
