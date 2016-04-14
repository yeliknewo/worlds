use std::fmt;
use std::error::Error;

use dorp::{
    IdManager, Id, IdType, Named, WorldErr, NamedErr
};

use core::{WEntity, WEntityErr, WScene, WMap, WMAP_NAME};

pub fn new_test_scene(manager: &mut IdManager) -> WEntity {
    let id = Id::new(manager, IdType::Entity);
    let scene = WScene::new(Box::new(|manager, world| {
        let wmap_id = {
            let id = Id::new(manager, IdType::Entity);
            let wmap = WMap::new();
            let named = match Named::new(WMAP_NAME, id, world) {
                Ok(named) => named,
                Err(err) => return Err(Box::new(SceneErr::Named("Named New WMAP_NAME Id World", err))),
            };
            if let Err(err) = world.add_entity(WEntity::new(id)
                .with_wmap(wmap)
                .with_named(named)
            ) {
                return Err(Box::new(SceneErr::World("World Add Entity", err)));
            }
        };
        println!("Test Scene Loaded");
        Ok(())
    }));
    WEntity::new(id)
    .with_scene(scene)
}

#[derive(Debug)]
enum SceneErr {
    World(&'static str, WorldErr),
    Named(&'static str, NamedErr),
    WEntity(&'static str, WEntityErr),
    Get(&'static str),
}

impl fmt::Display for SceneErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SceneErr::World(_, ref err) => err.fmt(f),
            SceneErr::Named(_, ref err) => err.fmt(f),
            SceneErr::WEntity(_, ref err) => err.fmt(f),
            SceneErr::Get(_) => write!(f, "Get was None"),
        }
    }
}

impl Error for SceneErr {
    fn description(&self) -> &str {
        match *self {
            SceneErr::World(_, ref err) => err.description(),
            SceneErr::Named(_, ref err) => err.description(),
            SceneErr::WEntity(_, ref err) => err.description(),
            SceneErr::Get(_) => "Get was None",
        }
    }
}
