use std::sync::{Arc};

use dorp::{
    Entity, IdManager, Id, Renderable, Named, Transform, TickCount, Window, SyncData,
    Renderers, DorpErr
};

use core::{WScene, WWorld, WMap, WCoords};
use components::{Chunk, Province, Overseer};

pub struct WEntity {
    renderable: Option<Box<Renderable>>,
    named: Option<Box<Named>>,
    transform: Option<Box<Transform>>,
    scene: Option<Box<WScene>>,
    wmap: Option<Box<WMap>>,
    wcoords: Option<Box<WCoords>>,
    chunk: Option<Box<Chunk>>,
    province: Option<Box<Province>>,
    overseer: Option<Box<Overseer>>,
    id: Id,
}

impl WEntity {
    pub fn new(id: Id) -> WEntity {
        WEntity {
            renderable: None,
            named: None,
            transform: None,
            scene: None,
            wmap: None,
            wcoords: None,
            chunk: None,
            province: None,
            overseer: None,
            id: id,
        }
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> WEntity {
        self.renderable = Some(Box::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> WEntity {
        self.named = Some(Box::new(named));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> WEntity {
        self.transform = Some(Box::new(transform));
        self
    }

    pub fn with_scene(mut self, scene: WScene) -> WEntity {
        self.scene = Some(Box::new(scene));
        self
    }

    pub fn with_wmap(mut self, wmap: WMap) -> WEntity {
        self.wmap = Some(Box::new(wmap));
        self
    }

    pub fn with_wcoords(mut self, wcoords: WCoords) -> WEntity {
        self.wcoords = Some(Box::new(wcoords));
        self
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> WEntity {
        self.chunk = Some(Box::new(chunk));
        self
    }

    pub fn with_province(mut self, province: Province) -> WEntity {
        self.province = Some(Box::new(province));
        self
    }

    pub fn with_overseer(mut self, overseer: Overseer) -> WEntity {
        self.overseer = Some(Box::new(overseer));
        self
    }

    pub fn get_wmap(&self) -> Option<&Box<WMap>> {
        self.wmap.as_ref()
    }

    pub fn get_province(&self) -> Option<&Box<Province>> {
        self.province.as_ref()
    }

    pub fn get_mut_province(&mut self) -> Option<&mut Box<Province>> {
        self.province.as_mut()
    }

    pub fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>> {
        self.renderable.as_mut()
    }

    pub fn get_mut_chunk(&mut self) -> Option<&mut Box<Chunk>> {
        self.chunk.as_mut()
    }
}

impl Entity<WEntity> for WEntity {
    fn tick(&self, tick_count: TickCount, delta_time: f64, world: Arc<WWorld>) -> Result<(), DorpErr> {
        if let Some(chunk) = self.chunk.as_ref() {
            if let Err(err) = chunk.tick(world) {
                return Err(DorpErr::Dorp("Chunk Tick World", Box::new(err)));
            }
        }
        Ok(())
    }

    fn tick_mut(&mut self, tick_count: TickCount, manager: &mut IdManager, world: &mut WWorld, sync_data: &mut SyncData) -> Result<(), DorpErr> {
        let id = self.get_id();
        if let Some(overseer) = self.overseer.as_mut() {
            overseer.tick_mut(tick_count);
        }
        if let Some(scene) = self.scene.as_mut() {
            if let Err(err) = scene.tick_mut(id, manager, world, sync_data) {
                return Err(DorpErr::Dorp("Scene Tick Mut Id Manager World", Box::new(err)));
            }
        }
        if let Some(chunk) = self.chunk.as_mut() {
            if let Err(err) = chunk.tick_mut() {
                return Err(DorpErr::Dorp("Chunk Tick Mut", Box::new(err)));
            }
        }
        if let Some(province) = self.province.as_mut() {
            if let Err(err) = province.tick_mut(world) {
                return Err(DorpErr::Dorp("Province Tick Mut", Box::new(err)));
            }
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), DorpErr> {
        if let Some(renderable) = self.renderable.as_mut() {
            if let Some(transform) = self.transform.as_mut() {
                if let Err(err) = transform.render(renderable) {
                    return Err(DorpErr::Dorp("Transform Render Renderable", Box::new(err)));
                }
            }
            if let Err(err) = renderable.render(window, sync_data, renderers) {
                return Err(DorpErr::Dorp("Renderable Render Window Syncdata Renderers", Box::new(err)));
            }
        }
        Ok(())
    }

    fn get_renderable(&self) -> Option<&Box<Renderable>> {
        self.renderable.as_ref()
    }

    fn get_named(&self) -> Option<&Box<Named>> {
        self.named.as_ref()
    }

    fn get_transform(&self) -> Option<&Box<Transform>> {
        self.transform.as_ref()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}
