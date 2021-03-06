extern crate dorp;
extern crate rand;

use dorp::{
    WindowBuilder, Vec2, IdManager
};

pub mod core;
pub mod entities;
pub mod components;

use core::{WGame};

fn main() {
    let mut manager = IdManager::new();
    let (mut window, resolution) = WindowBuilder::new()
        .with_title("Worlds".to_string())
        .build()
        .unwrap();
    let thread_count = 8;
    let mut game = WGame::new(thread_count, Vec2::from([resolution.0 as f32, resolution.1 as f32]));
    // let scene = entities::new_test_scene(&mut manager);
    let scene = entities::new_world_gen_scene(&mut manager);
    game.get_mut_world().unwrap().add_entity(scene);
    println!("Starting Run Loop");
    game.run(&mut window, &mut manager).unwrap();
}
