use std::time::{Duration, Instant};

use ggez::event::{self, KeyCode};
use ggez::graphics::{self, mint::Point2};
use ggez::{Context, GameResult};


// Here we define the size of our game board in terms of how many grid
// cells it will take up. We choose to make a 30 x 20 game board.
//const GRID_SIZE: (i16, i16) = (30, 20);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
//const GRID_CELL_SIZE: (i16, i16) = (32, 32);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
/*const SCREEN_SIZE: (u32, u32) = (
    GRID_SIZE.0 as u32 * GRID_CELL_SIZE.0 as u32,
    GRID_SIZE.1 as u32 * GRID_CELL_SIZE.1 as u32,
);*/

const SCREEN_SIZE: (u32, u32) = (800, 400);

// Here we're defining how many quickly we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
const UPDATES_PER_SECOND: f32 = 10.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;


// This is a trait that provides a modulus function that works for negative values
// rather than just the standard remainder op (%) which does not.
trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

// Here we implement our `ModuloSigned` trait for any type T which implements
// `Add` (the `+` operator) with an output type T and Rem (the `%` operator)
// that also has anout put type of T, and that can be cloned. These are the bounds
// that we need in order to implement a modulus function that works for negative numbers
// as well.
impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        // Because of our trait bounds, we can now apply these operators.
        (self.clone() % n.clone() + n.clone()) % n
    }
}

struct MainState {
    pos_x: f32,
    offset_x: f32,
    last_update: Instant,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { 
            pos_x: 0.0,
            offset_x: 10.0,
            last_update: Instant::now(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // First we check to see if enough time has elapsed since our last update based on
        // the update rate we defined at the top.
        let now = Instant::now();
        if now - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            // we update the state
            self.pos_x = self.pos_x.modulo(SCREEN_SIZE.0 as f32) + self.offset_x;
            
            // If we updated, we set our last_update to be now
            self.last_update = now;
        }
        // Finally we return `Ok` to indicate we didn't run into any errors
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let mesh = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                Point2::from([self.pos_x, SCREEN_SIZE.1 as f32 / 2.0]),
                100.0,
                0.1,
                [0.0, 0.0, 1.0, 1.0].into()
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    // key_down_event gets fired when a key gets pressed.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            KeyCode::Left  => { self.offset_x = -self.offset_x.abs() },
            KeyCode::Right => { self.offset_x = self.offset_x.abs() },
            _ => (),
        };
    }
}

fn main() -> GameResult {
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Sandbox", "Valentin Colin")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("SANDBOX"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()
        .expect("Failed to build ggez context");

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = MainState::new(&mut ctx)?;
    // And finally we actually run our game, passing in our context, event_loop and state.
    event::run(ctx, event_loop, state)
}
