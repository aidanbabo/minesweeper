// TODO 
// - MAKE AN ALERT FOR JEFFERY BECAUSE YOU LOVE HIM
// - BUG - when bomb is on far right side, the space directly to the left often doesn't get
// calculated properly
// - add smiley
//   - requires facial animations while clicking on flagged
// - add numbers
//   - need to create time and mines variables that are shared
//   - these to have to have numeric representations in the code
//     like how the open spaces have representations in the code.
//     OR not and just store it as a number
//   - TODO how tf to timers work in this language, std::Duration?
// - levels of difficulty
//   - look into the source code to see what the difficulties are
//     search for "Expert"
// - remove now-redudant parts of setting size passing

use piston::window::WindowSettings;
use piston::input::{RenderEvent};
use piston::event_loop::{Events, EventSettings};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

mod minesweeper;
mod minesweeper_controller;
mod minesweeper_view;

pub use crate::minesweeper::{MineSweeper, ROWS, COLS};
pub use crate::minesweeper_controller::MineSweeperController;
pub use crate::minesweeper_view::{MineSweeperView, MineSweeperViewSettings};

fn main() {
    // initialize custom classes to handle events and the like
    // model
    let  ms = MineSweeper::new();
    // controller
    let mut ms_c = MineSweeperController::new(ms);
    // view
    let settings = MineSweeperViewSettings::new(ROWS, COLS, 2.5);

    let width: f64 = settings.cols as f64 * settings.square_side + 2.0 * settings.border_long;
    let height: f64 = settings.rows as f64 * settings.square_side + 3.0 * settings.border_long + settings.smiley_side;

    // create window
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("Mine Sweeper", [width, height])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build()
        .expect("Could not create window");

    let mut event_settings = EventSettings::new();
    event_settings.lazy = true;
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let ms_v = MineSweeperView::new(settings.clone());
    
    // event loop
    while let Some(e) = events.next(&mut window) {

        // handle input event
        ms_c.event(settings.clone(), &e);

        // handle rendering
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                use graphics::clear;

                // clear screen and call draw function
                clear([0.8, 0.8, 0.8, 1.0], g);
                ms_v.draw(&ms_c, &c, g);
            });
        }
    }
}
