use std::fmt;
use std::error::Error;

use dorp::{Id, Vec4, OptErr, WorldErr};

use core::{WWorld, WEntityErr};

#[derive(Debug)]
pub struct Province {
    chunks: Vec<Id>,
    color: Vec4,
    dirty: bool,
}

impl Province {
    pub fn new(color: Vec4) -> Province {
        Province {
            chunks: vec!(),
            color: color,
            dirty: false,
        }
    }

    pub fn tick_mut(&mut self, world: &mut WWorld) -> Result<(), ProvinceErr> {
        if self.dirty {
            self.dirty = false;
            for chunk_id in self.chunks.iter() {
                match world.get_mut_entity_by_id(*chunk_id) {
                    OptErr::Full(chunk_entity) => {
                        match chunk_entity.get_mut_chunk() {
                            OptErr::Full(chunk) => {

                            },
                            OptErr::Empty => return Err(ProvinceErr::Get("Chunk Entity Get Mut Chunk")),
                            OptErr::Error(err) => return Err(ProvinceErr::WEntity("Chunk Entity Get Mut Chunk", err)),
                        }
                    },
                    OptErr::Empty => return Err(ProvinceErr::Get("World Get Mut Entity by Id Chunk")),
                    OptErr::Error(err) => return Err(ProvinceErr::World("World Get Mut Entity By Id Chunk", err)),
                }
            }
        }
        Ok(())
    }

    pub fn add_chunk(&mut self, chunk: Id) {
        self.chunks.push(chunk);
        self.dirty = true;
    }
}

#[derive(Debug)]
pub enum ProvinceErr {
    World(&'static str, WorldErr),
    WEntity(&'static str, WEntityErr),
    // Named(&'static str, NamedErr),

    Get(&'static str),
}

impl fmt::Display for ProvinceErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProvinceErr::World(_, ref err) => err.fmt(f),
            ProvinceErr::WEntity(_, ref err) => err.fmt(f),
            // ProvinceErr::Named(_, ref err) => err.fmt(f),
            ProvinceErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for ProvinceErr {
    fn description(&self) -> &str {
        match *self {
            ProvinceErr::World(_, ref err) => err.description(),
            ProvinceErr::WEntity(_, ref err) => err.description(),
            // ProvinceErr::Named(_, ref err) => err.description(),
            ProvinceErr::Get(_) => "Get was None",
        }
    }
}
