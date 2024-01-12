pub enum Shot {
  Bullseye,
  Hit(f64), // distance from center
  Miss,
}

impl Shot {
  pub fn points(&self) -> i32 {
      match *self {
          Shot::Bullseye => 5,
          Shot::Hit(distance) => {
              if distance < 3.0 {
                  2
              } else {
                  1
              }
          }
          Shot::Miss => 0,
      }
  }
}
