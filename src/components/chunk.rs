use std::sync::{Arc};

use dorp::{Id, DorpErr};

use core::{WCoords, WWorld, OVERSEER_NAME};
use components::{Province};

#[derive(Debug)]
pub struct Chunk {
    province_id: Id,
    neighbors: Vec<Id>,
    full_neighbors: Vec<WCoords>,
    empty_neighbors: Vec<WCoords>,
}

impl Chunk {
    pub fn new(my_id: Id, province_id: Id, province: &mut Province) -> Chunk {
        province.add_chunk(my_id);
        Chunk {
            province_id: province_id,
            neighbors: vec!(),
            full_neighbors: vec!(),
            empty_neighbors: vec!(),
        }
    }

    pub fn tick(&self, world: Arc<WWorld>) -> Result<(), DorpErr> {
        match world.get_entity_by_name(OVERSEER_NAME) {
            Some(overseer_entity) => match overseer_entity.get_wmap() {
                Some(wmap) => if wmap.is_dirty() {
                    // self.prep_update_neighbors(&wmap);
                },
                None => return Err(DorpErr::Base("Overseer entity get wmap was none")),
            },
            None => return Err(DorpErr::Base("World Get Entity by Name overseer name was none")),
        }
        Ok(())
    }
}
