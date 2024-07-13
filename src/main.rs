// !!! REMOVE FOR RELEASE !!!
#![allow(unused_imports, unused_variables)]


// imports
use ggez::{
	graphics::*,
	event::{self, EventHandler},
	conf::{Conf, Backend, WindowSetup, WindowMode, FullscreenType},
	glam::*,
	input::keyboard::{KeyCode, KeyMods, KeyInput},
	Context, ContextBuilder, GameResult
};

// module declarations
pub mod global_constants;
pub mod grid_drawer;
pub mod player;

// module imports
use global_constants as GlobConst;
use grid_drawer as GridDrawer;
use player::*;



// GAME STATE STRUCT
struct Game {
	quad_mesh: Mesh,		// temporary
	tile_image: Image,		// make a struct in grid drawer?
	player: Player,
}

impl Game {
	pub fn new(ctx: &mut Context) -> Game {
		Game {
			quad_mesh: Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, GlobConst::QUAD_SIZE, GlobConst::QUAD_SIZE), Color::WHITE).unwrap(),
			tile_image: Image::from_path(ctx, "/tile.png").unwrap(),
			player: Player::new(0, 0),
		}
	}
}

impl EventHandler for Game {
	fn update(self: &mut Game, ctx: &mut Context) -> GameResult {
		self.player.update(ctx)?;

		Ok(())
	}

	fn draw(self: &mut Game, ctx: &mut Context) -> GameResult {

		// get canvas, disable filtering, and dimensions
		let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
		canvas.set_sampler(Sampler::nearest_clamp());
		let (screen_w, screen_h) = ctx.gfx.size();
		
		// draw grid with (temporary) quad mesh
		GridDrawer::draw_grid(ctx, &mut canvas, &self.tile_image)?;

		// draw player
		self.player.draw(ctx, &mut canvas, &self.quad_mesh)?;

		// present
		canvas.finish(ctx)?;
		Ok(())
	}

	fn key_down_event(self: &mut Game, ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
		// call the players keydown function to make him move
		self.player.key_down(ctx, input, repeat)?;

		Ok(())
	}
}



// ========== MAIN FUNCTION ==========
fn main() {

	// setup window config
	let window_mode = WindowMode {
			width: 1920.0,
			height: 1080.0,
			maximized: true,
			fullscreen_type: FullscreenType::True,
			borderless: true,
			resizable: false,
			..Default::default()
	};

	// build context and event loop
	let (mut ctx, event_loop) = ContextBuilder::new("Nordic Grid Game", "NLAM")
		.window_mode(window_mode)
		.build()
		.expect("Couldn't create ggez context.");

	let game = Game::new(&mut ctx);

	event::run(ctx, event_loop, game);
}
