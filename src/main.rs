// TODO 
// - add border
//   - change constant to variables in MSVSettings
// - add smiley
//   - requires facial animations while clicking on flagged
//   - need to redesign mouse input, for now it ignores imput
//     in that region when checking for valid input
// - add numbers
//   - need to create time and mines variables that are shared
//   - these to have to have numeric representations in the code
//     like how the open spaces have representations in the code.
//     OR not and just store it as a number
//   - TODO how tf to timers work in this language, std::Duration?
// - levels of difficulty
//   - look into the source code to see what the difficulties are
//     search for "Expert"
// - click on the number jawn
//   - automatically clears all around it if there are that number
//     of flags around it
//   - if it runs into a bomb, you die... :)

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

// screen heigh and width, currently just enough to fill play area
// TODO make larger to accomodate new size
const WIDTH: f64 = 516.0;   // 31 tiles * 16 width + each border which is 10 each
const HEIGHT: f64 = 314.0;  // 16 tiles * 16 height + each border which is 10 each
                            // plus additional top space which is 28 and includes 
                            // another border of height 10

fn main() {
    // create window
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Mine Sweeper", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut event_settings = EventSettings::new();
    event_settings.lazy = true;
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    // initialize custom classes to handle events and the like
    // model
    let  ms = MineSweeper::new();
    // controller
    let mut ms_c = MineSweeperController::new(ms);
    // view
    let ms_v = MineSweeperView::new(MineSweeperViewSettings::new(ROWS, COLS));

    // event loop
    while let Some(e) = events.next(&mut window) {

        // handle input event
        ms_c.event(ms_v.settings.playtl, [31.0 * 16.0, 16.0 * 16.0], &e);

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
