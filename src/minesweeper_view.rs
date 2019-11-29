use graphics::{Context, Transformed, Image, rectangle::square};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;

use crate::minesweeper_controller::MineSweeperController;
use crate::minesweeper::{Content, Status};

/// Loads all textures and controls which are drawn by `MineSweeperView::draw()`.
struct Textures {
    /// Unmarked square
    pub blank: Texture,
    /// All textures with numbers and 0
    pub opens: Vec<Texture>,
    /// Revealed mine texture
    pub mine: Texture,
    /// Flagged square texture
    pub flagged: Texture,
    /// Questioned square texture
    pub questioned: Texture,
    /// Red bomb texture used for the bomb misclick
    pub bombdeath: Texture,
    /// A misflagged bomb texture for when the user guesses incorrectly
    pub bombmisflagged: Texture,
}

impl Textures {

    /// Loads all texture assets
    pub fn new() -> Textures {
        let ref texture_settings = TextureSettings::new();;
        let blank = Texture::from_path(Path::new("./assets/blank.gif"), texture_settings).expect("Could not find image");
        let mut opens: Vec<Texture> = Vec::with_capacity(9);
        for open in 0..9usize {
            opens.push(Texture::from_path(Path::new(&format!("./assets/open{}.gif", open)), texture_settings).expect("Could not find open image"));
        }

        let mine = Texture::from_path(Path::new("./assets/bombrevealed.gif"), texture_settings).expect("Could not find bomb image");
        let flagged = Texture::from_path(Path::new("./assets/bombflagged.gif"), texture_settings).expect("Could not find flagged");
        let questioned = Texture::from_path(Path::new("./assets/bombquestioned.gif"), texture_settings).expect("Could not find questioned");

        let bombdeath = Texture::from_path(Path::new("./assets/bombdeath.gif"), texture_settings).expect("Could not find bombdeath");
        let bombmisflagged = Texture::from_path(Path::new("./assets/bombmisflagged.gif"), texture_settings).expect("Could not find bombmisflagged");


        Textures { 
            blank,
            opens,
            mine,
            flagged,
            questioned,
            bombdeath,
            bombmisflagged,
        }
    }

    /// Retrieves the correct square texture depending on the game state.
    pub fn by_index(&self, controller: &MineSweeperController, row: usize, col: usize) -> &Texture {
        let square = &controller.minesweeper.get(row, col);
        if !controller.minesweeper.lost {
            match square.status {
                Status::Uncovered => {
                    match square.content {
                        Content::Zero => &self.opens[0],
                        Content::One => &self.opens[1],
                        Content::Two => &self.opens[2],
                        Content::Three => &self.opens[3],
                        Content::Four => &self.opens[4],
                        Content::Five => &self.opens[5],
                        Content::Six => &self.opens[6],
                        Content::Seven => &self.opens[7],
                        Content::Eight => &self.opens[8],
                        Content::Mine => &self.mine,
                    }
                },
                Status::Unmarked => &self.blank,
                Status::Flagged => &self.flagged,
                Status::Questioned => &self.questioned,
            }
        } else {
            match square.status {
                Status::Uncovered => {
                    match square.content {
                        Content::Zero => &self.opens[0],
                        Content::One => &self.opens[1],
                        Content::Two => &self.opens[2],
                        Content::Three => &self.opens[3],
                        Content::Four => &self.opens[4],
                        Content::Five => &self.opens[5],
                        Content::Six => &self.opens[6],
                        Content::Seven => &self.opens[7],
                        Content::Eight => &self.opens[8],
                        Content::Mine => &self.bombdeath,
                    }
                },
                Status::Unmarked => if square.content == Content::Mine { &self.mine } else { &self.blank },
                Status::Flagged  => if square.content == Content::Mine { &self.flagged } else { &self.bombmisflagged },
                Status::Questioned => if square.content == Content::Mine { &self.mine } else { &self.questioned },
            }
        }
    }
} 

/// Settings, certain distances and such are stores here
pub struct MineSweeperViewSettings {
    pub cell_size: f64,
    pub rows: usize,
    pub cols: usize,
}

impl MineSweeperViewSettings {
    pub fn new(rows: usize, cols: usize) -> Self {
        MineSweeperViewSettings {
            cell_size: 16.0,
            rows,
            cols,
        }
    }
}

/// Struct in charge of drawing to the screen.
pub struct MineSweeperView {
    pub settings: MineSweeperViewSettings,
    textures: Textures,
}

impl MineSweeperView {
    pub fn new(settings: MineSweeperViewSettings) -> Self {
        let textures = Textures::new();
        MineSweeperView { settings, textures }
    }

    /// Draws to the screen
    pub fn draw(
        &self, 
        controller: &MineSweeperController, 
        c: &Context,
        g: &mut GlGraphics
    )
    {
        let ref settings = self.settings;
        let image = Image::new().rect(square(0.0, 0.0, settings.cell_size));

        for i in 0..settings.rows{
            for j in 0..settings.cols {
                image.draw(self.textures.by_index(controller, i, j),
                           &c.draw_state,
                           c.transform.trans(j as f64 * settings.cell_size, i as f64 * settings.cell_size),
                           g);
            }
        }
    }
}
