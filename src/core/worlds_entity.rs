use std::sync::{Arc};
use std::error::Error;
use std::fmt;

use dorp::{
    Entity, IdManager, Id, Renderable, Named, Transform, TickCount, Window, SyncData,
    Renderers, RenderableErr, TransformErr, OptErr
};

use core::{WScene, WWorld, WMap, WCoords};
use components::{Chunk};

pub struct WEntity {
    renderable: Option<Arc<Renderable>>,
    named: Option<Arc<Named>>,
    transform: Option<Arc<Transform>>,
    scene: Option<Arc<WScene>>,
    wmap: Option<Arc<WMap>>,
    wcoords: Option<Arc<WCoords>>,
    chunk: Option<Arc<Chunk>>,
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

    pub fn get_mut_chunk(&mut self) -> OptErr<&mut Chunk, WEntityErr> {
        if let Some(chunk) = self.chunk.as_mut() {
            if let Some(chunk) = Arc::get_mut(chunk) {
                return OptErr::Full(chunk);
            } else {
                return OptErr::Error(WEntityErr::GetMut("Arc Get Mut Chunk"));
            }
        }
        return OptErr::Empty;
    }
}

impl Entity<WEntity> for WEntity {
    fn tick(&self, tick_count: Arc<TickCount>, delta_time: Arc<f64>, world: Arc<WWorld>) -> Result<(), Box<Error>> {
        Ok(())
    }

    fn tick_mut(&mut self, tick_count: TickCount, manager: &mut IdManager, world: &mut WWorld) -> Result<(), Box<Error>> {
        let id = self.get_id();
        if let Some(scene) = self.scene.as_mut() {
            if let Some(scene) = Arc::get_mut(scene) {
                if let Err(err) = scene.tick_mut(id, manager, world) {
                    return Err(Box::new(WEntityErr::Scene("Scene Tick Mut Id Manager World", err)));
                }
            } else {
                return Err(Box::new(WEntityErr::GetMut("Arc Get Mut Scene")));
            }
        }
        Ok(())
    }

    fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), Box<Error>> {
        if let Some(renderable) = self.renderable.as_mut() {
            if let Some(renderable) = Arc::get_mut(renderable) {
                if let Some(transform) = self.transform.as_mut() {
                    if let Some(transform) = Arc::get_mut(transform) {
                        if let Err(err) = transform.render(renderable) {
                            return Err(Box::new(WEntityErr::Transform("Transform Render Renderable", err)));
                        }
                    }
                }
                if let Err(err) = renderable.render(window, sync_data, renderers) {
                    return Err(Box::new(WEntityErr::Renderable("Renderable Render Window SyncData Renderers", err)));
                }
            } else {
                return Err(Box::new(WEntityErr::GetMut("Arc Get Mut Renderable")));
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

#[derive(Debug)]
pub enum WEntityErr {
    Renderable(&'static str, RenderableErr),
    Transform(&'static str, TransformErr),
    Scene(&'static str, Box<Error>),
    GetMut(&'static str),
}

impl fmt::Display for WEntityErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WEntityErr::Renderable(_, ref err) => err.fmt(f),
            WEntityErr::Transform(_, ref err) => err.fmt(f),
            WEntityErr::Scene(_, ref err) => err.fmt(f),
            WEntityErr::GetMut(_) => write!(f, "Get Mut was None"),
        }
    }
}

impl Error for WEntityErr {
    fn description(&self) -> &str {
        match *self {
            WEntityErr::Renderable(_, ref err) => err.description(),
            WEntityErr::Transform(_, ref err) => err.description(),
            WEntityErr::Scene(_, ref err) => err.description(),
            WEntityErr::GetMut(_) => "Get Mut was None",
        }
    }
}
