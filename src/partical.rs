use macroquad::{math::{vec2, Vec2}, prelude::rand, window::{screen_height, screen_width}};

const MASS_MODIFIER:f32 = 2.0;
const GRAVITY:f32 = 1.0;
const CHARGE:f32 = -1.0;

pub struct Partical {
  pub pos:Vec2, // current pos
  pub vel:Vec2, // direction of motion

  pub mass:f32, // mass of partical
  pub chr:f32, // charge of partical
  pub ang_mom:f32, // spin of partical
  // either clockwise (+) or ccw (-)
}
impl Partical {
  // always come in pairs to maintain equilibrium
  pub fn new() -> (Self,Self) {
    let pos= vec2(
      rand::gen_range(0.0, screen_width()),
      rand::gen_range(0.0, screen_height())
    );
    let vel= vec2(
      rand::gen_range(0.0, 10.),
      rand::gen_range(0.0, 10.)
    );

    let mass = 10.0;
    let chr = rand::gen_range(-100.0,100.0);
    let ang_mom= rand::gen_range(-100.0,100.0);
    (Self {
      pos,
      vel,
    
      mass,
      chr,
      ang_mom,
    },
    Self {
      pos,
      vel: vel * (-1.0),
    
      mass,
      chr:chr * (-1.0),
      ang_mom: ang_mom* (-1.0),
    }
  )
  }
  pub fn move_forward(&mut self) {
    self.pos += self.vel;
  }

}

impl Partical {
  pub fn get_size(&self) -> f32{
    let size= self.mass / MASS_MODIFIER;
    size
  }
  // force twrd the partical self
  pub fn gravitaional_force(&self, other:&Partical) -> Vec2 {
    let (d,distance) = self.displacment(other.pos);
    let direction = d/distance;
    if distance <= self.get_size() + other.get_size() { return Vec2::ZERO }
    let f = GRAVITY * self.mass * other.mass /(distance*distance);
    d * f // force times direction
  }
  pub fn electrostatic_force (&self,other:&Partical) -> Vec2 {
    let (d,distance) = self.displacment(other.pos);
    let direction = d/distance;
    if distance <= self.get_size() + other.get_size() { return Vec2::ZERO }
    let f = CHARGE * self.chr * other.chr /(distance*distance);
    d * f // force times direction
  }
  pub fn check_colliding(&self, other:&Partical) -> bool {
    let (_,distance) = self.displacment(other.pos);
    self.get_size() + other.get_size() <= distance
  }



  pub fn instantanious_collide(&mut self, other:&mut Partical) {
    let (v1,v2) = (self.vel, other.vel);
    let (m1,m2) = (self.mass, other.mass);
    // colision vector
    let col_norm = (self.pos - other.pos)/((self.pos - other.pos).length());
    // gets the vector that is inline with the collision (swap)
    let (perp_v1,perp_v2) = (v1.project_onto_normalized(col_norm),v2.project_onto_normalized(col_norm));
    // gets the vector of the velocity that is parralell to the collision (Don't change)
    let (parr_v1, parr_v2) = (v1-perp_v1, v2-perp_v2);
    // collide in 1 dimention, swap momenta
    let (perp_v1, perp_v2) = (
      ((m1-m2)*perp_v1 + 2.0*m2*perp_v2)/(m1+m2),
      ((m2-m1)*perp_v2 + 2.0*m1*perp_v1)/(m1+m2),
    );
    (self.vel, other.vel) = (perp_v1 + parr_v1, perp_v2 + parr_v2);
  }



  // redundent math, takes 2 positions and returns the vector between them and the magnitude
  fn displacment(&self, pos:Vec2) -> (Vec2, f32){
    let dx = self.pos.x - pos.x;
    let dy = self.pos.y - pos.y;
    let d = vec2(dx, dy);
    let magniude = d.length();
    (d,magniude)
  }

  fn kinetic_energy(&self) -> f32 {
    0.5 * self.mass * self.vel.length() * self.vel.length()
  }
}