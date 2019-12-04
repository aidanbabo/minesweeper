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
    pub fn event<E: GenericEvent>(&mut self, offset: [f64; 2], size: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(p) = e.mouse_cursor_args() {
            self.cursor_pos = p;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0];
            let y = self.cursor_pos[1];
            if x >= 244.0 && x <= 272.0 && y >= 10.0 && y <= 38.0 {
                self.reset();
                return;
            }
        }

        if self.minesweeper.lost { return }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space => {
                    let x = self.cursor_pos[0] - offset[0];
                    let y = self.cursor_pos[1] - offset[1];
        
                    if x >= 0.0 && x < size[0] && y >= 1.0 && y < size[1] { 
                        let col = (x / size[0] * self.minesweeper.cols as f64) as usize;
                        let row = (y / size[1] * self.minesweeper.rows as f64) as usize;
                        self.mark(row, col);
                    }
                },
                _ => {},
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0] - offset[0];
            let y = self.cursor_pos[1] - offset[1];

            if x >= 0.0 && x < size[0] && y >= 1.0 && y < size[1] { 
                let col = (x / size[0] * self.minesweeper.cols as f64) as usize;
                let row = (y / size[1] * self.minesweeper.rows as f64) as usize;
                let ref status = self.minesweeper.get(row, col).status;
                if status == &Status::Unmarked  || status == &Status::Questioned {
                    self.reveal(row, col);
                } else if status == &Status::Uncovered {
                    self.clear_around(row, col);
                }
            }
        }
    }

    fn clear_around(&mut self, row: usize, col: usize) {
        let ref square = self.minesweeper.get(row, col);
        let around = self.flags_around(row, col);
        let i: u8 = square.content.clone().into();
        if i != 0 && i == around {
            if row > 0 {
                self.reveal(row-1, col);
                if col > 0 {
                    self.reveal(row-1, col-1);
                }
                if col < self.minesweeper.cols - 1 {
                    self.reveal(row-1, col+1);
                }
            }
            if row < self.minesweeper.rows - 1 {
                self.reveal(row+1, col);
                if col > 0 {
                    self.reveal(row+1, col-1);
                }
                if col < self.minesweeper.cols - 1 {
                    self.reveal(row+1, col+1);
                }
            }
            if col > 0 { self.reveal(row, col-1); }
            if col < self.minesweeper.cols - 1 { self.reveal(row, col+1); }
        }
    }

    fn reveal(&mut self, row: usize, col: usize) {
        let square = self.minesweeper.get_mut(row, col);
        // If flagged, don't click
        if square.status == Status::Flagged || square.status == Status::Uncovered { return }

        // Set status to uncovred, if not already set
        square.status = Status::Uncovered;
        // This is the recursive call for clearing blank spaces
        // In this case, the content must be zero, and it must currently be covered
        if square.content == Content::Zero { 
            if row > 0 {
                self.reveal(row-1, col);
                if col > 0 {
                    self.reveal(row-1, col-1);
                }
                if col < self.minesweeper.cols - 1 {
                    self.reveal(row-1, col+1);
                }
            }
            if row < self.minesweeper.rows - 1 {
                self.reveal(row+1, col);
                if col > 0 {
                    self.reveal(row+1, col-1);
                }
                if col < self.minesweeper.cols - 1 {
                    self.reveal(row+1, col+1);
                }
            }
            if col > 0 { self.reveal(row, col-1); }
            if col < self.minesweeper.cols - 1 { self.reveal(row, col+1); }
        // if it is a mine, lose
        } else if square.content == Content::Mine {
            self.lose();
        }
    }

    fn flags_around(&self, row: usize, col: usize) -> u8 {
        let ref field = self.minesweeper.field;
        let flag = Status::Flagged;
        let mut count = 0;
            if row > 0 {
                if field[row-1][col].status == flag { count += 1}
                if col > 0 {
                    if field[row-1][col-1].status == flag { count += 1}
                }
                if col < self.minesweeper.cols - 1 {
                    if field[row-1][col+1].status == flag { count += 1}
                }
            }
            if row < self.minesweeper.rows - 1 {
                    if field[row+1][col].status == flag { count += 1}
                if col > 0 {
                    if field[row+1][col-1].status == flag { count += 1}
                }
                if col < self.minesweeper.cols - 1 {
                    if field[row+1][col+1].status == flag { count += 1}
                }
            }
            if col > 0 { if field[row][col-1].status == flag { count += 1 } }
            if col < self.minesweeper.cols - 1 { if field[row][col+1].status == flag { count +=1 } }
        count
    }

    fn mark(&mut self, row: usize, col: usize) {
        let square = self.minesweeper.get_mut(row, col);
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
