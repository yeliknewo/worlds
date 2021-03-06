use dorp::{
    Id, IdManager, IdType, Transform, Renderable, Mat4, Vec3,
    DorpErr, Entity
};

use dorp::graphics::solid_color::{Vertex};

use core::{WEntity, WCoords, WWorld};
use components::{Chunk};

pub fn new_chunk(manager: &mut IdManager, renderable: &Renderable, zoom: Vec3, location: &WCoords, province_id: Id, world: &mut WWorld) -> Result<WEntity, DorpErr> {
    let id = Id::new(manager, IdType::Entity);
    let mut renderable = renderable.clone();
    let mut chunk = {
        let mut province_entity = match world.get_mut_entity_by_id(province_id) {
            Some(province_entity) => province_entity,
            None => return Err(DorpErr::Base("World Get Entity by Id province id was none")),
        };
        let province_id = province_entity.get_id();
        let province = match province_entity.get_mut_province() {
            Some(province) => province,
            None => return Err(DorpErr::Base("Province Entity Get mut Province was none")),
        };
        match renderable.get_mut_solid_color() {
            Some(renderable) => {
                renderable.set_model_id(Id::new(manager, IdType::Matrix));
                renderable.set_model(Mat4::identity());
                renderable.set_color_id(province.get_color_id());
            },
            None => return Err(DorpErr::Base("Get Mut Solid Color was none")),
        }
        Chunk::new(id, province_id, province)
    };
    let mut transform = Transform::new();
    transform.set_position(Vec3::from([location.get_x() as f32, location.get_y() as f32, 0.0]) * zoom);
    transform.set_scalation(zoom);
    chunk.add_neighbor_coords(WCoords::new(location.get_x() + 1, location.get_y()));
    chunk.add_neighbor_coords(WCoords::new(location.get_x() - 1, location.get_y()));
    chunk.add_neighbor_coords(WCoords::new(location.get_x(), location.get_y() + 1));
    chunk.add_neighbor_coords(WCoords::new(location.get_x(), location.get_y() - 1));
    Ok(
        WEntity::new(id)
        .with_renderable(renderable)
        .with_transform(transform)
        .with_chunk(chunk)
    )
}

pub fn new_chunk_renderable(manager: &mut IdManager, base: &Renderable) -> Result<Renderable, DorpErr> {
    let mut renderable = base.clone();
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
