use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[derive(Component)]
pub struct MovesRandomly {
  pub speed: f64,
}

#[derive(Component)]
pub struct Renderable {
    /// The name of the texture to be rendered
    pub tex_name: String,
    // Width of src
    pub i_w: u32,
    // Height of src
    pub i_h: u32,
    // Width of dest
    pub o_w: u32,
    // Height of dest
    pub o_h: u32,
    // Offset number of widths to crop
    pub frame: u32,
    // Max frame offset before 
    pub total_frames: u32,
    // Rotation of imagae to display
    pub rot: f64
}

#[derive(Component)]
pub struct Player {}

// Define a system for handling collision detection
pub struct CollisionDetectionSystem;

impl<'a> System<'a> for CollisionDetectionSystem {
  type SystemData = (Entities<'a>, ReadStorage<'a, Position>, ReadStorage<'a, Size>);

  fn run(&mut self, (entities, positions, sizes): Self::SystemData) {
    // Iterate over all pairs of entities with position and size components
    for (e1, pos1, size1) in (&entities, &positions, &sizes).join() {
      for (e2, pos2, size2) in (&entities, &positions, &sizes).join() {
        if e1 == e2 {
          // skip collision with self
          continue;
        }
        // Check if the two entities are colliding
        if pos1.x < pos2.x + size2.w &&
          pos1.x + size1.w > pos2.x &&
          pos1.y < pos2.y + size2.h &&
          pos1.y + size1.h > pos2.y
        {
          let thing = pos1.x;
          // Handle collision
          println!("Collision detected between entities! {thing}");
        }
      }
    }
  }
}
