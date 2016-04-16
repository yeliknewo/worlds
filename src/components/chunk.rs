use dorp::{Id};

use components::{Province};

#[derive(Debug)]
pub struct Chunk {
    province_id: Id,
    neighbors: Vec<Id>,
}

impl Chunk {
    pub fn new(my_id: Id, province_id: Id, province: &mut Province) -> Chunk {
        province.add_chunk(my_id);
        Chunk {
            province_id: province_id,
            neighbors: vec!(),
        }
    }
}
