use dorp::{Id, IdManager, IdType, Vec4, OptErr, SyncData, DorpErr};

use core::{WWorld};

#[derive(Debug)]
pub struct Province {
    chunks: Vec<Id>,
    color_id: Id,
    dirty: bool,
}

impl Province {
    pub fn new(manager: &mut IdManager, color_id: Id) -> Province {
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

    pub fn tick_mut(&mut self, world: &mut WWorld, sync_data: &mut SyncData) -> Result<(), DorpErr> {
        if self.dirty {
            self.dirty = false;
            for chunk_id in self.chunks.iter() {
                match world.get_mut_entity_by_id(*chunk_id) {
                    OptErr::Full(chunk_entity) => {
                        match chunk_entity.get_mut_renderable() {
                            OptErr::Full(renderable) => match renderable.get_mut_solid_color() {
                                Some(renderable) => {
                                    renderable.set_color_id(self.color_id);
                                },
                                None => return Err(DorpErr::Base("Renderable Get Mut Solid Color was none")),
                            },
                            OptErr::Empty => return Err(DorpErr::Base("Chunk Entity Get Mut Renderable was none")),
                            OptErr::Error(err) => return Err(DorpErr::Dorp("Chunk Entity Get Mut Renderable", Box::new(err))),
                        }
                    },
                    OptErr::Empty => return Err(DorpErr::Base("World Get Mut Entity by Id Chunk was none")),
                    OptErr::Error(err) => return Err(DorpErr::Dorp("World Get Mut Entity By Id Chunk", Box::new(err))),
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
