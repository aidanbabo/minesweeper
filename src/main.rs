// TODO 
// - add border
//   - requires ofset variable in cell calculations
//   - requires new WIDTH and HEIGHT variables
// - add smiley
//   - requires facial animations while clicking on flagged
//   - might require `.lazy(true)` for gif to play properly

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
const WIDTH: f64 = 496.0;
const HEIGHT: f64 = 256.0;

fn main() {
    // create window
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Mine Sweeper", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
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
        ms_c.event([31.0 * 16.0, 16.0 * 16.0], &e);

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
