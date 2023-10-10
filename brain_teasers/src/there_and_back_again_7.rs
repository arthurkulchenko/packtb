use std::f32::const::PI;

pub struct Degrees(pub f32);
pub struct Radianas(pub f32);

impl Degrees {
  pub fn new(angle: f32) -> Self {
    Self(angle)
  }
}

impl From<Degrees> for Radianas {
  fn from(item: Degrees) -> Self {
    Self(item.0 * PI / 180.0)
  }
}

pub fn call() {
  let one_eighty_degrees = Degrees::new(180.0);
  let one_eighty_radians : Radianas = one_eighty_degrees.into();
  println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}
