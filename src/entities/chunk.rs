use std::sync::{Arc};

use dorp::{
    Id, IdManager, IdType, Transform, Renderable, Mat4, Vec3,
    DorpErr, Entity, OptErr
};

use dorp::graphics::solid_color::{Vertex};

use core::{WEntity, WCoords};
use components::{Chunk};

pub fn new_chunk(manager: &mut IdManager, renderable: Arc<Renderable>, zoom: Vec3, location: Arc<WCoords>, province_entity: &mut WEntity) -> Result<WEntity, DorpErr> {
    let id = Id::new(manager, IdType::Entity);
    let mut renderable = Renderable::new_from(renderable);
    let province_id = province_entity.get_id();
    let province = match province_entity.get_mut_province() {
        OptErr::Full(province) => province,
        OptErr::Empty => return Err(DorpErr::Base("Province Entity Get mut Province was none")),
        OptErr::Error(err) => return Err(DorpErr::Dorp("Province Entity Get Mut Province", Box::new(err))),
    };
    match renderable.get_mut_solid_color() {
        Some(renderable) => {
            renderable.set_model_id(Id::new(manager, IdType::Matrix));
            renderable.set_model(Mat4::identity());
            renderable.set_color_id(province.get_color_id());
        },
        None => return Err(DorpErr::Base("Get Mut Solid Color was none")),
    }
    let mut transform = Transform::new();
    transform.set_position(Vec3::from([location.get_x() as f32, location.get_y() as f32, 0.0]) * zoom);
    transform.set_scalation(zoom);
    let chunk = Chunk::new(id, province_id, province);
    Ok(
        WEntity::new(id)
        .with_renderable(renderable)
        .with_transform(transform)
        .with_chunk(chunk)
    )
}

pub fn new_chunk_renderable(manager: &mut IdManager, base: Arc<Renderable>) -> Result<Renderable, DorpErr> {
    let mut renderable = Renderable::new_from(base);
    {
        let mut solid_color = match renderable.get_mut_solid_color() {
            Some(solid_color) => solid_color,
            None => return Err(DorpErr::Base("Renderable Get Mut Solid Color was none")),
        };

        solid_color.set_vertex_id(Id::new(manager, IdType::Vertex));
        solid_color.set_vertices(vec!(
            Vertex::new([-0.5, -0.5, 0.0]),
            Vertex::new([0.5, -0.5, 0.0]),
            Vertex::new([0.5, 0.5, 0.0]),
            Vertex::new([-0.5, 0.5, 0.0]),
        ));

        solid_color.set_index_id(Id::new(manager, IdType::Index));
        solid_color.set_indices(vec!(
            0, 1, 2,
            2, 3, 0,
        ));

        solid_color.set_model_id(Id::new(manager, IdType::Matrix));
        solid_color.set_model(Mat4::identity());
    }
    Ok(
        renderable
    )
}
