use specs::{World, WorldExt, Builder};
use specs::Join;
use std::collections::HashMap;
use rand::Rng;

const PLAYER_SPEED: f64 = 3.0;
const SHIFT_SPEED: f64 = 6.0;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    if crate::utils::is_key_pressed(&key_manager, "R") {
      ecs.delete_all();
      // Reset the world to first state
      load_world(ecs);
    }

    let mut positions = ecs.write_storage::<crate::components::Position>();
    let players = ecs.read_storage::<crate::components::Player>();
    let random_movers = ecs.read_storage::<crate::components::MovesRandomly>();

    let mut speed = PLAYER_SPEED;

    if crate::utils::is_key_pressed(&key_manager, "Left Shift") {
      speed = SHIFT_SPEED;
    }

    for (_, pos) in (&random_movers, &mut positions).join() {
      let mut rng = rand::thread_rng();

      pos.rot += rng.gen_range(-5.0..5.0) * PLAYER_SPEED;
      let radians = pos.rot.to_radians();

      let xchange = 0.5 * radians.sin();
      let ychange = 0.5 * radians.cos();
      
      let newx = pos.x + xchange;
      let newy = pos.y - ychange;

      pos.x = newx;
      pos.y = newy;
    }
    
    for (_, pos) in (&players, &mut positions).join() {
        if crate::utils::is_key_pressed(&key_manager, "D") {
            pos.rot += speed/2.0;
        }
        if crate::utils::is_key_pressed(&key_manager, "A") {
            pos.rot -= speed/2.0;
        }
        if crate::utils::is_key_pressed(&key_manager, "W") {
          let radians = pos.rot.to_radians();
          pos.x += speed * radians.sin();
          pos.y -= speed * radians.cos();
        }
        if crate::utils::is_key_pressed(&key_manager, "S") {
          let radians = pos.rot.to_radians();
          pos.x -= speed * radians.sin();
          pos.y += speed * radians.cos(); 
        }

        if pos.rot > 360.0 {
            pos.rot -= 360.0;
        }
        if pos.rot < 0.0 {
            pos.rot += 360.0;
        }
    }
}

pub fn load_world(ecs: &mut World) {
  load_characters(ecs);
  create_colliders(ecs);
}

fn create_colliders(ecs: &mut World) {
  ecs.create_entity()
    .with(crate::components::Position{ x:0.0, y:0.0, rot: 0.0})
    .with(crate::components::Size{ w: 5.0, h: 600.0 })
    .build();

  ecs.create_entity()
    .with(crate::components::Position{ x:5.0, y:0.0, rot: 0.0})
    .with(crate::components::Size{ w: 795.0, h: 5.0 })
    .build();

  ecs.create_entity()
    .with(crate::components::Position{ x:795.0, y:5.0, rot: 0.0})
    .with(crate::components::Size{ w: 5.0, h: 595.0 })
    .build();

  ecs.create_entity()
    .with(crate::components::Position{ x:5.0, y:600.0, rot: 0.0})
    .with(crate::components::Size{ w: 795.0, h: 5.0 })
    .build();
}

fn load_characters(ecs: &mut World) {
  ecs.create_entity()
  .with(crate::components::Position{ x:350.0, y:250.0, rot: 0.0})
  .with(crate::components::Renderable{
      tex_name: String::from("img/avatar.png"),
      i_w: 100,
      i_h: 100,
      o_w: 100,
      o_h: 100,
      frame: 0,
      total_frames: 1,
      rot: 0.0
  })
  .with(crate::components::Player {})
  .with(crate::components::Size{ w: 90.0, h: 90.0 })
  .build();

ecs.create_entity()
  .with(crate::components::Position{ x:400.0, y:300.0, rot: 0.0})
  .with(crate::components::Renderable{
      tex_name: String::from("img/avatar.png"),
      i_w: 100,
      i_h: 100,
      o_w: 100,
      o_h: 100,
      frame: 0,
      total_frames: 1,
      rot: 0.0
  })
  .with(crate::components::MovesRandomly{ speed: 10.0 })
  .with(crate::components::Size{ w: 90.0, h: 90.0 })
  .build();
}
