use graphics::{Context, Transformed, Image, rectangle::square};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;

use crate::minesweeper_controller::MineSweeperController;
use crate::minesweeper::{Content, Status};

struct Smiley {
    pub normal: Texture,
}

impl Smiley {
    fn new(texture_settings: &TextureSettings) -> Self {
        let normal = Texture::from_path(Path::new("./assets/normalsmiley.gif"), texture_settings).expect("Could not find normal smiley");
        Smiley {
            normal,
        }
    }
}

struct Border {
    pub topbottom: Texture,
    pub leftright: Texture,
    pub topleft: Texture,
    pub topright: Texture,
    pub bottomleft: Texture,
    pub bottomright: Texture,
    pub leftjoin: Texture,
    pub rightjoin: Texture,
}

impl Border {
    fn new(texture_settings: &TextureSettings) -> Self {
        let topbottom = Texture::from_path(Path::new("./assets/topbottom.gif"), texture_settings).expect("Could not find top bottom border");
        let leftright = Texture::from_path(Path::new("./assets/leftright.gif"), texture_settings).expect("Could not find left right border");
        let topleft = Texture::from_path(Path::new("./assets/topleft.gif"), texture_settings).expect("Could not find top left border");
        let topright = Texture::from_path(Path::new("./assets/topright.gif"), texture_settings).expect("Could not find top right border");
        let bottomleft = Texture::from_path(Path::new("./assets/bottomleft.gif"), texture_settings).expect("Could not find bottom left border");
        let bottomright = Texture::from_path(Path::new("./assets/bottomright.gif"), texture_settings).expect("Could not find bottom right border");
        let leftjoin = Texture::from_path(Path::new("./assets/leftjoin.gif"), texture_settings).expect("Could not find left join");
        let rightjoin = Texture::from_path(Path::new("./assets/rightjoin.gif"), texture_settings).expect("Could not find right join");
        Border {
            topbottom,
            leftright,
            topleft,
            topright,
            bottomleft,
            bottomright,
            leftjoin,
            rightjoin,
        }
    }
}

/// Loads all textures and controls which are drawn by `MineSweeperView::draw()`.
struct Textures {
    /// Unmarked square
    pub blank: Texture,
    /// All textures with numbers and 0
    pub opens: [Texture; 9],
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
    /// Empty background space
    pub empty: Texture,
    /// All border pieces
    pub border: Border,
    /// All smiley textures
    pub smiley: Smiley,
}

impl Textures {

    /// Loads all texture assets
    pub fn new() -> Textures {
        let ref texture_settings = TextureSettings::new();
        let blank = Texture::from_path(Path::new("./assets/blank.gif"), texture_settings).expect("Could not find image");

        // unsafe way to initialized an array incrementally
        // this will include lots of comments so I know what is going on
        use std::mem::{self, MaybeUninit};
        // size is, as per usual, hardcoded, but easy to change through this constant
        // however, we can no longer use [a, b, c] syntax as that is independent of this constant
        const SIZE: usize = 9;

        let opens = {

            // create an uninitialized array of `MaybeUninit`. The `assume_init` is safe because
            // the type we are claiming to have initialized here is a bunch of `MaybeUninit`s,
            // which do not require initialization.
            let mut opens: [MaybeUninit<Texture>; SIZE] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            // dropping a `MaybeUninit` does nothing. Thus using raw pointer assignment
            // instead of `ptr::write` does not cause the old uninitialized value to be dropped.
            for open in 0..SIZE {
                opens[open] = MaybeUninit::new(Texture::from_path(Path::new(&format!("./assets/open{}.gif", open)), texture_settings).expect("Clould not find open image"));
            }

            // everything is initialized.
            // transmute the array to the initialized type
            unsafe { mem::transmute::<_, [Texture; SIZE]>(opens) }

        };

        let mine = Texture::from_path(Path::new("./assets/bombrevealed.gif"), texture_settings).expect("Could not find bomb image");
        let flagged = Texture::from_path(Path::new("./assets/bombflagged.gif"), texture_settings).expect("Could not find flagged");
        let questioned = Texture::from_path(Path::new("./assets/bombquestioned.gif"), texture_settings).expect("Could not find questioned");

        let bombdeath = Texture::from_path(Path::new("./assets/bombdeath.gif"), texture_settings).expect("Could not find bombdeath");
        let bombmisflagged = Texture::from_path(Path::new("./assets/bombmisflagged.gif"), texture_settings).expect("Could not find bombmisflagged");

        let empty = Texture::from_path(Path::new("./assets/empty.gif"), texture_settings).expect("Could not find empty space");
        let border = Border::new(texture_settings);
        let smiley = Smiley::new(texture_settings);

        Textures { 
            blank,
            opens,
            mine,
            flagged,
            questioned,
            bombdeath,
            bombmisflagged,
            empty,
            border,
            smiley,
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
    pub playtl: [f64; 2],
}

impl MineSweeperViewSettings {
    pub fn new(rows: usize, cols: usize) -> Self {
        MineSweeperViewSettings {
            cell_size: 16.0,
            rows,
            cols,
            playtl: [10.0, 48.0]
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

        // draw corner and join pieces
        let side = 10.0;
        let image = Image::new().rect(square(0.0, 0.0, side));
        // top left
        image.draw(&self.textures.border.topleft,
                   &c.draw_state,
                   c.transform,
                   g);
        // top right
        image.draw(&self.textures.border.topright,
                   &c.draw_state,
                   c.transform.trans(506.0, 0.0),
                   g);
        // bottom left
        image.draw(&self.textures.border.bottomleft,
                   &c.draw_state,
                   c.transform.trans(0.0, 304.0),
                   g);
        // bottom right
        image.draw(&self.textures.border.bottomright,
                   &c.draw_state,
                   c.transform.trans(506.0, 304.0),
                   g);
        // left join
        image.draw(&self.textures.border.leftjoin,
                   &c.draw_state,
                   c.transform.trans(0.0, 38.0),
                   g);
        // right join
        image.draw(&self.textures.border.rightjoin,
                   &c.draw_state,
                   c.transform.trans(506.0, 38.0),
                   g);

        // draw horizontal borders - 124 total
        let width = 4.0;
        let height = 10.0;
        let image = Image::new().rect([0.0, 0.0, width, height]);
        for i in 0..124 {
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), 0.0),
                       g);
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), 38.0),
                       g);
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), 304.0),
                       g);
        }

        // draw vertical borders in two segments
        let width = 10.0;
        let height = 4.0;
        let image = Image::new().rect([0.0, 0.0, width, height]);
        for i in 0..7 {
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(0.0, 10.0 + (i as f64 * height)),
                       g);
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(506.0, 10.0 + (i as f64 * height)),
                       g);
        }
        for i in 0..64 {
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(0.0, 48.0 + (i as f64 * height)),
                       g);
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(506.0, 48.0 + (i as f64 * height)),
                       g);
        }

        let side = 28.0;
        let image = Image::new().rect(square(0.0, 0.0, side));
        image.draw(&self.textures.smiley.normal,
                   &c.draw_state,
                   c.transform.trans(506.0/2.0 - 14.0, 10.0),
                   g);

        // draw field
        let image = Image::new().rect(square(0.0, 0.0, settings.cell_size));
        for i in 0..settings.rows{
            for j in 0..settings.cols {
                let pos = [
                    (j as f64 * settings.cell_size) + settings.playtl[0],
                    (i as f64 * settings.cell_size) + settings.playtl[1],
                ];
                image.draw(self.textures.by_index(controller, i, j),
                           &c.draw_state,
                           c.transform.trans(pos[0], pos[1]),
                           g);
            }
        }
    }
}
