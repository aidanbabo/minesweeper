use rand::{Rng, thread_rng};
use std::ops::{Add, AddAssign};
// use std::fmt;

/// Contains all possible variants of what a certain square can hold
#[derive(Clone, PartialEq)]
pub enum Content {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Mine,
}

impl Into<u8> for  Content {
    fn into(self) -> u8 {
        match self {
            Content::Zero => 0,
            Content::One => 1,
            Content::Two => 2,
            Content::Three => 3,
            Content::Four => 4,
            Content::Five => 5,
            Content::Six => 6,
            Content::Seven => 7,
            Content::Eight => 8,
            Content::Mine => 9,
        }
    }
}

impl From<u8> for Content {
    fn from(n: u8) -> Self {
        match n {
            0 => Content::Zero,
            1 => Content::One,
            2 => Content::Two,
            3 => Content::Three,
            4 => Content::Four,
            5 => Content::Five,
            6 => Content::Six,
            7 => Content::Seven,
            8 => Content::Eight,
            9 => Content::Mine,
            _ => Content::Mine,
        }
    }
}

impl Default for Content {
    fn default() -> Self { Content::Zero }
}

impl Add for Content {
    type Output = Content;

    fn add(self, rhs: Self) -> Self::Output {
        (Into::<u8>::into(self) + Into::<u8>::into(rhs)).into()
    }
}

impl AddAssign for Content {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

/// Contains all possible statuses of a square
#[derive(PartialEq)]
pub enum Status {
    Uncovered,
    Unmarked,
    Flagged,
    Questioned,
}

impl Default for Status {
    fn default() -> Self { Status::Unmarked }
}

/// The struct containing information about each position on the game board
/// Can either be `Uncovered` and show the `content` inside
/// or `Flagged`, `Questioned`, or `Unmarked`.
#[derive(Default, PartialEq)]
pub struct Square {
    pub content: Content,
    pub status: Status,
}

impl Square {
    fn new() -> Self {
        Square { status: Status::Unmarked, content: Content::Zero }
    }
}

pub const ROWS: usize = 16;
pub const COLS: usize = 31;

/// The type of array used to store all the squares
type Field = [[Square; COLS]; ROWS]; 

pub struct MineSweeper {
    pub field: Field,
    pub rows: usize,
    pub cols: usize,
    pub lost: bool,
}

impl MineSweeper {
    pub fn new() -> Self {
        let mut field: Field = Default::default();
        for i in 0..ROWS {
            for j in 0..COLS {
                field[i][j] = Square::new();
            }
        }
        Self::populate(&mut field);
        Self::calculate(&mut field);
        MineSweeper {
            field,
            rows: ROWS,
            cols: COLS,
            lost: false,
        }
    }

    /// Retrieves a mutalble reference to a square
    pub fn get<T: Into<usize>>(&self, row: T, col: T) -> &Square {
        &self.field[row.into()][col.into()]
    }

    /// Retrieves an immutable reference to a square
    pub fn get_mut<T: Into<usize>>(&mut self, row: T, col: T) -> &mut Square {
        &mut self.field[row.into()][col.into()]
    }

    fn populate(field: &mut Field) {
        let mut mines = 99;
        let mut rng = thread_rng();
        let empty = Square::new();
        while mines > 0 {
            let (row, col) = (rng.gen_range(0, field.len()), rng.gen_range(0, field[0].len()));
            if field[row][col] == empty {
                field[row][col].content = Content::Mine;
                mines -= 1;
            }
        }
    }

    fn calculate(field: &mut Field) {

        // middle section
        for i in 1..(ROWS-1) {
            for j in 1..(COLS-1) {
               if field[i][j].content == Content::Mine {
                   field[i-1][j-1].content += 1.into();
                   field[i-1][j].content += 1.into();
                   field[i-1][j+1].content += 1.into();
                   field[i][j-1].content += 1.into();
                   field[i][j+1].content += 1.into();
                   field[i+1][j-1].content += 1.into();
                   field[i+1][j].content += 1.into();
                   field[i+1][j+1].content += 1.into();
                }
            }
        }

        // left and right
        for i in 1..(ROWS-1) {
            if field[i][0].content == Content::Mine {
                field[i-1][0].content += 1.into();
                field[i-1][1].content += 1.into();
                field[i][1].content += 1.into();
                field[i+1][0].content += 1.into();
                field[i+1][1].content += 1.into();
            }
            if field[i][COLS-1].content == Content::Mine {
                field[i-1][COLS-2].content += 1.into();
                field[i-1][COLS-1].content += 1.into();
                field[i+1][COLS-2].content += 1.into();
                field[i+1][COLS-1].content += 1.into();
            }
        }

        // top and bottom
        for j in 1..30 {
            if field[0][j].content == Content::Mine {
                field[0][j-1].content += 1.into();
                field[0][j+1].content += 1.into();
                field[1][j-1].content += 1.into();
                field[1][j].content += 1.into();
                field[1][j+1].content += 1.into();
            } 
            if field[ROWS-1][j].content == Content::Mine {
                field[ROWS-2][j-1].content += 1.into();
                field[ROWS-2][j].content += 1.into();
                field[ROWS-2][j+1].content += 1.into();
                field[ROWS-1][j-1].content += 1.into();
                field[ROWS-1][j+1].content += 1.into();
            } 
        }

        // edges
        if field[0][0].content == Content::Mine {
            field[0][1].content += 1.into();
            field[1][1].content += 1.into();
            field[1][0].content += 1.into();
        }
        if field[0][COLS-1].content == Content::Mine {
            field[0][COLS-2].content += 1.into();
            field[1][COLS-2].content += 1.into();
            field[1][COLS-1].content += 1.into();
        }
        if field[ROWS-1][0].content == Content::Mine {
            field[ROWS-1][1].content += 1.into();
            field[ROWS-2][1].content += 1.into();
            field[ROWS-2][0].content += 1.into();
        }
        if field[ROWS-1][COLS-1].content == Content::Mine {
            field[ROWS-1][COLS-2].content += 1.into();
            field[ROWS-2][COLS-2].content += 1.into();
            field[ROWS-2][COLS-1].content += 1.into();
        }
    }
}
