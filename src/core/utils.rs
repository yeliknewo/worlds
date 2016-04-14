use dorp::{
    Scene, Game, World, Map2d, Map2dCoords
};

use core::{WEntity};

pub type WScene = Scene<WEntity>;
pub type WGame = Game<WEntity>;
pub type WWorld = World<WEntity>;
pub type WMap = Map2d<WMapCoordSize>;
pub type WCoords = Map2dCoords<WMapCoordSize>;
pub type WMapCoordSize = i32;
pub static WMAP_NAME: &'static str = "WMapName";
