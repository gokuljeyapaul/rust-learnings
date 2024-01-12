// A coordinate of where an Arrow hit
#[derive(Debug)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }
}