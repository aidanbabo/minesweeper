use piston::input::GenericEvent;

use crate::MineSweeper;
use crate::minesweeper::{Content, Status};

/// Handles all events, communicating between input and data
pub struct MineSweeperController {
    pub minesweeper: MineSweeper,
    pub cursor_pos: [f64; 2],
}

impl MineSweeperController {
    pub fn new(minesweeper: MineSweeper) -> Self {
        MineSweeperController { 
            minesweeper,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles an event
    pub fn event<E: GenericEvent>(&mut self, size: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if e.press_args() == Some(Button::Keyboard(Key::R)) {
            println!("I pressed R!");
            self.reset();
            return;
        }

        if self.minesweeper.lost { return }

        if let Some(p) = e.mouse_cursor_args() {
            self.cursor_pos = p;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    let x = self.cursor_pos[0];
                    let y = self.cursor_pos[1];
        
                    if x >= 0.0 && x < size[0] && y >= 1.0 && y < size[1] { 
                        let cell_x = (x / size[0] * self.minesweeper.cols as f64) as usize;
                        let cell_y = (y / size[1] * self.minesweeper.rows as f64) as usize;
                        self.mark(cell_x, cell_y);
                    }
                },
                _ => {},
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0];
            let y = self.cursor_pos[1];

            if x >= 0.0 && x < size[0] && y >= 1.0 && y < size[1] { 
                let cell_x = (x / size[0] * self.minesweeper.cols as f64) as usize;
                let cell_y = (y / size[1] * self.minesweeper.rows as f64) as usize;
                self.reveal(cell_x, cell_y);
            }
        }
    }

    fn reveal(&mut self, x: usize, y: usize) {
        let square = self.minesweeper.get_mut(y, x);
        if square.status != Status::Uncovered && square.status != Status::Flagged {
            square.status = Status::Uncovered;
            if square.content == Content::Zero {
                if x > 0 {
                    self.reveal(x-1, y);
                    if y > 0 {
                        self.reveal(x-1, y-1);
                    }
                    if y < self.minesweeper.rows - 1 {
                        self.reveal(x-1, y+1);
                    }
                }
                if x < self.minesweeper.cols - 1 {
                    self.reveal(x+1, y);
                    if y > 0 {
                        self.reveal(x+1, y-1);
                    }
                    if y < self.minesweeper.rows - 1 {
                        self.reveal(x+1, y+1);
                    }
                }
                if y > 0 { self.reveal(x, y-1); }
                if y < self.minesweeper.rows - 1 { self.reveal(x, y+1); }
            // TODO most of the game logic
            //      - what about bomb?
            } else if square.content == Content::Mine {
                self.lose();
            }
        }
    }

    fn mark(&mut self, x: usize, y: usize) {
        let square = self.minesweeper.get_mut(y, x);
        square.status = match square.status {
            Status::Uncovered => Status::Uncovered,
            Status::Unmarked => Status::Flagged,
            Status::Flagged => Status::Questioned,
            Status::Questioned => Status::Unmarked,
        };

    }

    fn lose(&mut self) {
        self.minesweeper.lost = true;
    }

    fn reset(&mut self) {
        self.minesweeper = MineSweeper::new();
    }
}
