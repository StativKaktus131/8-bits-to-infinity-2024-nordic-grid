use ggez::{
	graphics::*,
	event::{self, EventHandler},
	glam::*,
	input::keyboard::{KeyCode, KeyMods, KeyInput},
	Context, GameResult
};

use crate::grid_drawer as GridDrawer;
use crate::global_constants as GlobConst;


pub struct Player {
	pos: Vec2,		// position in the grid
	screen_pos: Vec2,	// position on screen
	size: Vec2,		// temporary: later sprite/image size
}

impl Player {
	
	pub fn new(x: u32, y: u32) -> Player {
		Player {
			pos: vec2(x as f32, y as f32),
			screen_pos: vec2(0.0, 0.0),
			size: vec2(20.0, 30.0),
		}
	}

	pub fn update(self: &mut Player, ctx: &mut Context) -> GameResult {
		// convert grid position to screen position
		GridDrawer::grid_pos_to_screen(ctx, &self.pos, &mut self.screen_pos)?;
		
		// pivot at the feet
		self.screen_pos -= vec2(self.size.x * 0.5, self.size.y) * GlobConst::SCALE;

		Ok(())
	}

	pub fn draw(self: &mut Player, ctx: &mut Context, canvas: &mut Canvas, quad_mesh: &Mesh) -> GameResult {

		// maybe save player rect in the struct?
		let draw_param = DrawParam::default()
			.dest_rect(Rect::new(
				self.screen_pos.x,
				self.screen_pos.y,
				self.size.x / GlobConst::QUAD_SIZE * GlobConst::SCALE,
				self.size.y / GlobConst::QUAD_SIZE * GlobConst::SCALE
			));

		canvas.draw(quad_mesh, draw_param);

		Ok(())
	}

	pub fn key_down(self: &mut Player, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
		
		// move the players position in the grid
		match input.keycode {
			Some(KeyCode::Up) => self.pos += vec2(0.0, -1.0),
			Some(KeyCode::Down) => self.pos += vec2(0.0, 1.0),
			Some(KeyCode::Left) => self.pos += vec2(-1.0, 0.0),
			Some(KeyCode::Right) => self.pos += vec2(1.0, 0.0),
			_ => (),
		}

		// keep player in grid
		self.pos = self.pos.clamp(Vec2::ZERO, Vec2::ONE * (GridDrawer::TILES_PER_ROW - 1.0));

		Ok(())
	}
}
