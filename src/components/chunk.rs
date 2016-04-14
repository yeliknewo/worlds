use dorp::{Id};

#[derive(Debug)]
pub struct Chunk {
    province_id: Id,
}

impl Chunk {
    pub fn new(province_id: Id) -> Chunk {
        Chunk {
            province_id: province_id,
        }
    }
}
