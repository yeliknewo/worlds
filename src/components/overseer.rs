use dorp::{Vec3, TickCount};

pub struct Overseer {
    zoom: Vec3,
    last_tick: TickCount,
    dirty: bool,
}

impl Overseer {
    pub fn new(zoom: Vec3, tick_count: TickCount) -> Overseer {
        Overseer {
            zoom: zoom,
            last_tick: tick_count,
            dirty: true,
        }
    }

    pub fn tick_mut(&mut self, tick_count: TickCount) {
        if tick_count - self.last_tick > 1 {
            self.dirty = false;
        }
    }

    pub fn set_zoom(&mut self, zoom: Vec3) {
        self.zoom = zoom;
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn get_zoom(&self) -> Vec3 {
        self.zoom
    }
}
