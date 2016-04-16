use dorp::{Id, IdManager, IdType, Vec4, SyncData, DorpErr};

use core::{WWorld};

#[derive(Debug)]
pub struct Province {
    chunks: Vec<Id>,
    color_id: Id,
    dirty: bool,
}

impl Province {
    pub fn new(color_id: Id) -> Province {
        Province {
            chunks: vec!(),
            color_id: color_id,
            dirty: false,
        }
    }

    pub fn new_with_color(manager: &mut IdManager, sync_data: &mut SyncData, color: Vec4) -> Province {
        let id = Id::new(manager, IdType::Color);
        sync_data.set_vec4(id, color);
        Province {
            chunks: vec!(),
            color_id: id,
            dirty: false,
        }
    }

    pub fn tick_mut(&mut self, world: &mut WWorld) -> Result<(), DorpErr> {
        if self.dirty {
            self.dirty = false;
            for chunk_id in self.chunks.iter() {
                match world.get_mut_entity_by_id(*chunk_id) {
                    Some(chunk_entity) => {
                        match chunk_entity.get_mut_renderable() {
                            Some(renderable) => match renderable.get_mut_solid_color() {
                                Some(renderable) => {
                                    renderable.set_color_id(self.color_id);
                                },
                                None => return Err(DorpErr::Base("Renderable Get Mut Solid Color was none")),
                            },
                            None => return Err(DorpErr::Base("Chunk Entity Get Mut Renderable was none")),
                        }
                    },
                    None => return Err(DorpErr::Base("World Get Mut Entity by Id Chunk was none")),
                }
            }
        }
        Ok(())
    }

    pub fn add_chunk(&mut self, chunk: Id) {
        self.chunks.push(chunk);
        self.dirty = true;
    }

    pub fn get_color_id(&self) -> Id {
        self.color_id
    }
}
