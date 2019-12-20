use graphics::{Context, Transformed, Image, rectangle::square};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;

use crate::minesweeper_controller::MineSweeperController;
use crate::minesweeper::{Content, Status};

struct Smiley {
    pub normal: Texture,
    pub x_eyes: Texture,
    pub sunglasses: Texture,
}

impl Smiley {
    fn new(texture_settings: &TextureSettings) -> Self {
        let normal = Texture::from_path(Path::new("./assets/normalsmiley.gif"), texture_settings).expect("Could not find normal smiley");
        let x_eyes = Texture::from_path(Path::new("./assets/deadsmiley.gif"), texture_settings).expect("Could not find dead smiley");
        let sunglasses = Texture::from_path(Path::new("./assets/sunglassessmiley.gif"), texture_settings).expect("Could not find sunglasses smiley");
        Smiley {
            normal,
            x_eyes,
            sunglasses,
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
    /// All big red numbers
    pub numbers: [Texture; 10],
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

        let numbers = {
            let mut numbers: [MaybeUninit<Texture>; 10] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            for number in 0..10 {
                numbers[number] = MaybeUninit::new(Texture::from_path(Path::new(&format!("./assets/time{}.gif", number)), texture_settings).expect("Could not find number image"));
            }
            unsafe { mem::transmute::<_, [Texture; 10]>(numbers) }
        };

        let mine = Texture::from_path(Path::new("./assets/bombrevealed.gif"), texture_settings).expect("Could not find bomb image");
        let flagged = Texture::from_path(Path::new("./assets/bombflagged.png"), texture_settings).expect("Could not find flagged");
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
            numbers,
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

    pub fn smiley(&self, controller: &MineSweeperController) -> &Texture {
        if controller.minesweeper.won { &self.smiley.sunglasses }
        else if controller.minesweeper.lost { &self.smiley.x_eyes }
        else { &self.smiley.normal }
    }

} 

/// Settings, certain distances and such are stores here
#[derive(Clone, Copy)]
pub struct MineSweeperViewSettings {
    pub square_side: f64,
    pub rows: usize,
    pub cols: usize,
    pub border_long: f64,
    pub border_short: f64,
    pub smiley_side: f64,
    pub time_height: f64,
    pub time_width: f64,
    pub scr_width: f64,
    pub scr_height: f64,
}

impl MineSweeperViewSettings {
    pub fn new(rows: usize, cols: usize, scale: f64) -> Self {
        let square_side = 16.0 * scale;
        let border_long = 10.0 * scale;
        let border_short = 4.0 * scale;
        let smiley_side = 28.0 * scale;
        let time_height = 23.0 * scale;
        let time_width = 13.0 * scale;
        let scr_width = border_long * 2.0 + cols as f64 * square_side;
        let scr_height = border_long * 3.0 + rows as f64 * square_side + smiley_side;
        MineSweeperViewSettings {
            square_side,
            rows,
            cols,
            border_long,
            border_short,
            smiley_side,
            time_height,
            time_width,
            scr_width,
            scr_height,
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
        let mid_divider = settings.border_long + settings.smiley_side;
        let bot_divider = settings.border_long * 2.0 + settings.smiley_side + settings.rows as f64 * settings.square_side;
        let far_right = settings.border_long + settings.cols as f64 * settings.square_side;

        // draw corner and join pieces
        let side = settings.border_long;
        let image = Image::new().rect(square(0.0, 0.0, side));
        // top left
        image.draw(&self.textures.border.topleft,
                   &c.draw_state,
                   c.transform,
                   g);
        // top right
        image.draw(&self.textures.border.topright,
                   &c.draw_state,
                   c.transform.trans(far_right, 0.0),
                   g);
        // bottom left
        image.draw(&self.textures.border.bottomleft,
                   &c.draw_state,
                   c.transform.trans(0.0, bot_divider),
                   g);
        // bottom right
        image.draw(&self.textures.border.bottomright,
                   &c.draw_state,
                   c.transform.trans(far_right, bot_divider),
                   g);
        // left join
        image.draw(&self.textures.border.leftjoin,
                   &c.draw_state,
                   c.transform.trans(0.0, mid_divider),
                   g);
        // right join
        image.draw(&self.textures.border.rightjoin,
                   &c.draw_state,
                   c.transform.trans(far_right, mid_divider),
                   g);

        // draw horizontal borders - 124 total
        let width = settings.border_short;
        let height = settings.border_long;
        let image = Image::new().rect([0.0, 0.0, width, height]);
        for i in 0..124 {
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), 0.0),
                       g);
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), mid_divider),
                       g);
            image.draw(&self.textures.border.topbottom,
                       &c.draw_state,
                       c.transform.trans(height + (i as f64 * width), bot_divider),
                       g);
        }

        // draw vertical borders in two segments
        let width = settings.border_long;
        let height = settings.border_short;
        let image = Image::new().rect([0.0, 0.0, width, height]);
        let amount = (settings.smiley_side / settings.border_short) as usize;
        for i in 0..amount {
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(0.0, settings.border_long + (i as f64 * height)),
                       g);
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(far_right, settings.border_long + (i as f64 * height)),
                       g);
        }
        let below_join = settings.border_long * 2.0 + settings.smiley_side;
        let amount = (settings.rows as f64 * settings.square_side / settings.border_short) as usize;
        for i in 0..amount {
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(0.0, below_join + (i as f64 * height)),
                       g);
            image.draw(&self.textures.border.leftright,
                       &c.draw_state,
                       c.transform.trans(far_right, below_join + (i as f64 * height)),
                       g);
        }

        let width = settings.time_width;
        let height = settings.time_height;
        let image = Image::new().rect([0.0, 0.0, width, height]);


        let side = settings.smiley_side;
        let image = Image::new().rect(square(0.0, 0.0, side));
        let texture = self.textures.smiley(controller);
        image.draw(texture,
                   &c.draw_state,
                   c.transform.trans(settings.scr_width/2.0 - settings.smiley_side/2.0, settings.border_long),
                   g);

        // draw field
        let image = Image::new().rect(square(0.0, 0.0, settings.square_side));
        let offset = [
            self.settings.border_long,
            self.settings.border_long * 2.0 + self.settings.smiley_side,
        ];
        for i in 0..settings.rows{
            for j in 0..settings.cols {
                let pos = [
                    (j as f64 * settings.square_side) + offset[0],
                    (i as f64 * settings.square_side) + offset[1],
                ];
                image.draw(self.textures.by_index(controller, i, j),
                           &c.draw_state,
                           c.transform.trans(pos[0], pos[1]),
                           g);
            }
        }
    }
}
