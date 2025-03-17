use crate::partical::{self, Partical};
use macroquad::prelude::*;

pub struct Universe {
  matter:Vec<Partical>,
  
}
impl Universe {
  pub fn new(n:u32) -> Self {
    // n is num of particals
    let mut matter:Vec<Partical> = Vec::new();
    // makes all the partical pairs
    for _ in 0..n/2 {
      let (a,b) = Partical::new();
      matter.push(a);
      matter.push(b);
    }
    Universe {
      matter
    }
  }

}
// Mechanics
impl Universe {
  fn total_force(&self, chosen:&Partical) -> f32 {
    let mut force = Vec2::ZERO;
    for i in &self.matter {
      force += i.electrostatic_force(chosen) + i.gravitaional_force(chosen);
    }
    0.0
  }
  fn collision(&mut self) {
    let mut collided: Vec<(usize, usize)> = Vec::new();
    for i in 1..self.matter.len() {
      let mut a = &self.matter[i];
      let mut b = &self.matter[i+1];
      if a.check_colliding(b) {collided.push((i,i+1));}
    }
  }
}


// Graphics
impl Universe {
  pub fn display(&self) {
    for i in &self.matter {
      let color = match i.chr {
        s if s > 0.0 => Color::new(0.0, 1.0, 0.0, s/100.0), // Green for pos
        s if s < 0.0 => Color::new(1.0, 0.0, 0.0, -s/100.0), // Red for neg
        _ => GRAY, // Gray for neutral
      };
      draw_circle(i.pos.x, i.pos.y, i.get_size(), color);
    }
  }
}