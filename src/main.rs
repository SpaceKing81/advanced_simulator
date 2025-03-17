use macroquad::prelude::*;
use partical::Partical;
use universe::Universe;
// use std::collections::{HashMap, HashSet};


mod partical;
mod universe;
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
