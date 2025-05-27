use macroquad::prelude::*;
use universe::Universe;
// use std::collections::{HashMap, HashSet};
/*
IDEA: EVERYTHING ONLY OPERATES ON COLLITIONS!!!!
2 TYPES OF PARTICALS, FORCE MEDIATORS AND PHYSICAL
THUS, PARTICALS ONLY CHECK TO SEE IF THERE ARE ANY AJACENT PARTICALS. ONLY NEED 1 FOR LOOP,
AND THEN CAN UPDATE ALL FORCE CARRIERS AFTER

loop structure:
    - Update all particles loop, release 8 force carriers in the 8 directions
    - Update all force carriers, y=e^(-x) decay, round 0.05 strength to 0
        - Force Carriers act as waves, and sit within a matrix-field built of a hashmap
        - Only need to update age, check if the age is too old for killing
        - Math the strength when read by another partical
        - If read, they become absorbed by the partical, bestowing the encoded momentum as well
    - All particals stored in a matrix, cannot occupy same spot
    - All FC's stored in a hashmap individual to their force, can occupy the same position

*/

mod partical;
mod universe;
mod carrier;
mod consts;
fn window_conf() -> Conf {
    Conf {
        window_title: "Brain Simulation".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}
const PARTICAL_COUNT:u32 = 10;
const IDEAL_TPS:f64 = 70.0;

#[macroquad::main(window_conf)]
async fn main() {
    
    // Initialize the World
    println!("Starting simulation...");
    let mut universe = Universe::new(PARTICAL_COUNT);
    // let anchor = Vec2::new(screen_width()/2.0, screen_height()/2.0);
    println!("Initialized. Entering continuous operations...");
    let mut ticks = 0.0;
    // Main loop
    loop {
        // Handle input
        if is_key_down(KeyCode::Escape) {
            println!("Terminating...");
            break;
        }
        loop {
            // Main simulation logic


            let time = if get_time() == 0.0 {0.02} else {get_time()};
            if ticks/time < IDEAL_TPS || is_key_down(KeyCode::Escape){ break; }
        }
        // Clear the screen
        clear_background(BLACK);

        // Draw
        universe.display();
        // Draw FPS and other info
        draw_text(
            &format!("TPS: {}, FPS: {}", (ticks/get_time()).round(), get_fps()),
            screen_width() - 200.,
            20.,
            20.,
            WHITE,
        );
        ticks += 1.0;
        // Render the frame
        next_frame().await;
    }
}


/*

*/
