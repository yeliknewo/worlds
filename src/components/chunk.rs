use std::sync::{Arc, RwLock};

use dorp::{Id, DorpErr};

use core::{WCoords, WWorld, WMap, OVERSEER_NAME};
use components::{Province};

#[derive(Debug)]
struct Changes {
    new_neighbors_coords: Vec<WCoords>,
    new_neighbors_id: Vec<Id>,
    dirty: bool,
}

impl Changes {
    fn new() -> Changes {
        Changes {
            new_neighbors_coords: vec!(),
            new_neighbors_id: vec!(),
            dirty: false,
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    province_id: Id,
    neighbors: Vec<Id>,
    full_neighbors: Vec<WCoords>,
    empty_neighbors: Vec<WCoords>,
    changes: Arc<RwLock<Changes>>,
}

impl Chunk {
    pub fn new(my_id: Id, province_id: Id, province: &mut Province) -> Chunk {
        province.add_chunk(my_id);
        Chunk {
            province_id: province_id,
            neighbors: vec!(),
            full_neighbors: vec!(),
            empty_neighbors: vec!(),
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn get_empty_neighbors(&self) -> &Vec<WCoords> {
        &self.empty_neighbors
    }

    pub fn add_neighbor_coords(&mut self, neighbor_coords: WCoords) {
        self.empty_neighbors.push(neighbor_coords);
    }

    pub fn update_neighbors(&mut self) -> Result<(), DorpErr> {
        match self.changes.write() {
            Ok(mut changes) => {
                if changes.dirty {
                    for neighbor in changes.new_neighbors_id.iter() {
                        self.neighbors.push(*neighbor);
                    }
                    changes.new_neighbors_id.clear();
                    for neighbor in changes.new_neighbors_coords.iter() {
                        self.full_neighbors.push(WCoords::new(neighbor.get_x(), neighbor.get_y()));
                    }
                    changes.new_neighbors_coords.clear();
                    changes.dirty = false;
                }
            },
            Err(_) => return Err(DorpErr::Base("Self Changes Write error")),
        }
        Ok(())
    }

    pub fn prep_update_neighbors(&self, wmap: &WMap) -> Result<(), DorpErr> {
        for neighbor in self.empty_neighbors.iter() {
            if wmap.get(neighbor.get_x(), neighbor.get_y()).is_some() {
                match self.changes.write() {
                    Ok(mut changes) => {
                        changes.new_neighbors_coords.push(WCoords::new(neighbor.get_x(), neighbor.get_y()));
                        changes.new_neighbors_id.push(match wmap.get(neighbor.get_x(), neighbor.get_y()) {
                            Some(id) => id,
                            None => return Err(DorpErr::Base("WMap get Neighbor X neighbor y was none")),
                        });
                        changes.dirty = true;
                    },
                    Err(_) => return Err(DorpErr::Base("Self Changes Write error")),
                }
            }
        }
        match self.changes.write() {
            Ok(mut changes) => {
                changes.new_neighbors_coords.sort();
                changes.new_neighbors_coords.dedup();
                changes.new_neighbors_id.sort();
                changes.new_neighbors_id.dedup();
            },
            Err(_) => return Err(DorpErr::Base("Self Changes Write error")),
        }
        Ok(())
    }

    pub fn tick(&self, world: Arc<WWorld>) -> Result<(), DorpErr> {
        match world.get_entity_by_name(OVERSEER_NAME) {
            Some(overseer_entity) => match overseer_entity.get_wmap() {
                Some(wmap) => if wmap.is_dirty() {
                    match self.prep_update_neighbors(&wmap) {
                        Ok(()) => (),
                        Err(err) => return Err(DorpErr::Dorp("Prep Update Neighbors WMap", Box::new(err))),
                    }
                },
                None => return Err(DorpErr::Base("Overseer entity get wmap was none")),
            },
            None => return Err(DorpErr::Base("World Get Entity by Name overseer name was none")),
        }
        Ok(())
    }

    pub fn tick_mut(&mut self) -> Result<(), DorpErr> {
        self.update_neighbors()
    }
}
