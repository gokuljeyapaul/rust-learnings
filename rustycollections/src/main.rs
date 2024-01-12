mod shot;
mod coord;

use shot::Shot;
use coord::Coord;

// Silence some warnings that could distract from the exercise
#[allow(unused_variables, unused_mut, dead_code)]

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    for coord in &arrow_coords {
        coord.print_description();
        
        let distance = coord.distance_from_center();

        let shot = if distance < 1.0 {
            Shot::Bullseye
        } else if distance <= 5.0 {
            Shot::Hit(distance)
        } else {
            Shot::Miss
        };

        shots.push(shot);
    }

    let mut total = 0;
    
    for shot in &shots {
        total = total + shot.points()
    }

    println!("Final point total is: {}", total);
}



// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}