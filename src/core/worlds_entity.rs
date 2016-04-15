use std::sync::{Arc};

use dorp::{
    Entity, IdManager, Id, Renderable, Named, Transform, TickCount, Window, SyncData,
    Renderers, OptErr, DorpErr
};

use core::{WScene, WWorld, WMap, WCoords};
use components::{Chunk, Province};

pub struct WEntity {
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    scene: Option<Arc<WScene>>,
    wmap: Option<Arc<WMap>>,
    wcoords: Option<Arc<WCoords>>,
    chunk: Option<Arc<Chunk>>,
    province: Option<Arc<Province>>,
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
            id: id,
        }
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> WEntity {
        self.renderable = Some(Arc::new(renderable));
        self
    }

    pub fn with_named(mut self, named: Named) -> WEntity {
        self.named = Some(Arc::new(named));
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> WEntity {
        self.transform = Some(Arc::new(transform));
        self
    }

    pub fn with_scene(mut self, scene: WScene) -> WEntity {
        self.scene = Some(Arc::new(scene));
        self
    }

    pub fn with_wmap(mut self, wmap: WMap) -> WEntity {
        self.wmap = Some(Arc::new(wmap));
        self
    }

    pub fn with_wcoords(mut self, wcoords: WCoords) -> WEntity {
        self.wcoords = Some(Arc::new(wcoords));
        self
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> WEntity {
        self.chunk = Some(Arc::new(chunk));
        self
    }

    pub fn with_province(mut self, province: Province) -> WEntity {
        self.province = Some(Arc::new(province));
        self
    }

    pub fn get_province(&self) -> Option<Arc<Province>> {
        self.province.clone()
    }

    pub fn get_mut_province(&mut self) -> OptErr<&mut Province, DorpErr> {
        if let Some(province) = self.province.as_mut() {
            if let Some(province) = Arc::get_mut(province) {
                return OptErr::Full(province);
            }
            return OptErr::Error(DorpErr::Base("Arc Get Mut Province was none"));
        }
        return OptErr::Empty;
    }

    pub fn get_mut_renderable(&mut self) -> OptErr<&mut Renderable, DorpErr> {
        if let Some(renderable) = self.renderable.as_mut() {
            if let Some(renderable) = Arc::get_mut(renderable) {
                return OptErr::Full(renderable);
            }
            return OptErr::Error(DorpErr::Base("Arc Get Mut Renderable was none"));
        }
        return OptErr::Empty;
    }

    pub fn get_mut_chunk(&mut self) -> OptErr<&mut Chunk, DorpErr> {
        if let Some(chunk) = self.chunk.as_mut() {
            if let Some(chunk) = Arc::get_mut(chunk) {
                return OptErr::Full(chunk);
            } else {
                return OptErr::Error(DorpErr::Base("Arc Get Mut Chunk was none"));
            }
        }
        return OptErr::Empty;
    }
}

impl Entity<WEntity> for WEntity {
    fn tick(&self, tick_count: Arc<TickCount>, delta_time: Arc<f64>, world: Arc<WWorld>) -> Result<(), DorpErr> {
        Ok(())
    }

    fn tick_mut(&mut self, tick_count: TickCount, manager: &mut IdManager, world: &mut WWorld, sync_data: &mut SyncData) -> Result<(), DorpErr> {
        let id = self.get_id();
        if let Some(scene) = self.scene.as_mut() {
            if let Some(scene) = Arc::get_mut(scene) {
                if let Err(err) = scene.tick_mut(id, manager, world, sync_data) {
                    return Err(DorpErr::Dorp("Scene Tick Mut Id Manager World", Box::new(err)));
                }
            } else {
                return Err(DorpErr::Base("Arc Get Mut Scene"));
            }
        }
        if let Some(province) = self.province.as_mut() {
            if let Some(province) = Arc::get_mut(province) {
                if let Err(err) = province.tick_mut(world, sync_data) {
                    return Err(DorpErr::Dorp("Province Tick Mut", Box::new(err)));
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), DorpErr> {
        if let Some(renderable) = self.renderable.as_mut() {
            if let Some(renderable) = Arc::get_mut(renderable) {
                if let Some(transform) = self.transform.as_mut() {
                    if let Some(transform) = Arc::get_mut(transform) {
                        if let Err(err) = transform.render(renderable) {
                            return Err(DorpErr::Dorp("Transform Render Renderable", Box::new(err)));
                        }
                    }
                }
                if let Err(err) = renderable.render(window, sync_data, renderers) {
                    return Err(DorpErr::Dorp("Renderable Render Window SyncData Renderers", Box::new(err)));
                }
            } else {
                return Err(DorpErr::Base("Arc Get Mut Renderable was none"));
            }
        }
        Ok(())
    }

    fn get_renderable(&self) -> Option<Arc<Renderable>> {
        self.renderable.clone()
    }

    fn get_named(&self) -> Option<Arc<Named>> {
        self.named.clone()
    }

    fn get_transform(&self) -> Option<Arc<Transform>> {
        self.transform.clone()
    }

    fn get_id(&self) -> Id {
        self.id
    }
}
