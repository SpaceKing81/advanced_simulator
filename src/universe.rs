use std::collections::HashMap;
use macroquad::prelude::*;

use crate::partical::Partical;
use crate::carrier::FC;
use crate::consts::*;

pub struct Universe {
  matter: Vec<Partical>,
  spatial_map: HashMap<(i32, i32), Vec<usize>>,
  cell_size: f32,
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
      matter,
      spatial_map:HashMap::new(),
      cell_size:CELL_SIZE,
    }
  }
  fn get_cell(&self, pos: Vec2) -> (i32, i32) {
    (
      (pos.x / self.cell_size) as i32,
      (pos.y / self.cell_size) as i32,
    )
  }

  fn update_spatial_map(&mut self) {
    self.spatial_map.clear();
    for (i, partical) in self.matter.iter().enumerate() {
      let cell = self.get_cell(partical.pos);
      self.spatial_map.entry(cell).or_default().push(i);
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
    self.update_spatial_map();
    let mut checked = std::collections::HashSet::new();

    for (&(x, y), indices) in &self.spatial_map {
      for dx in -1..=1 {
        for dy in -1..=1 {
          if let Some(other_indices) = self.spatial_map.get(&(x + dx, y + dy)) {
            for &i in indices {
              for &j in other_indices {
                if i >= j || checked.contains(&(j, i)) {
                  continue;
                }
                let (a, b) = self.matter.split_at_mut(j.max(i));
                let (a, b) = if i < j {
                  (&mut a[i], &mut b[0])
                } else {
                  (&mut a[j], &mut b[0])
                };
                if a.check_colliding(b) {
                  a.instantanious_collide(b);
                  checked.insert((i, j));
                }
              }
            }
          }
        }
      }
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