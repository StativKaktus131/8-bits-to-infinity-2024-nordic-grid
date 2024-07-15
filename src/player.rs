use ggez::{
	graphics::*,
	event::{self, EventHandler},
	glam::*,
	input::keyboard::{KeyCode, KeyMods, KeyInput},
	input::mouse::MouseButton,
	Context, GameResult
};

use crate::grid_drawer as GridDrawer;
use crate::global_constants as GlobConst;
use crate::math::sin;

use std::f32::consts::PI as PI;


const NEIGHBORS: [(i32, i32); 12] = [(0, -2), (-1, -1), (0, -1), (1, -1), (-2, 0), (-1, 0), (1, 0), (2, 0), (-1, 1), (0, 1), (1, 1), (0, 2)];

const COLORS: [[u8; 3]; 3] = [
	[75, 202, 50],
	[200, 60, 40],
	[54, 107, 186],
];
const BAR_PXS: [f32; 5] = [100.0 / 255.0, 220.0 / 255.0, 161.0 / 255.0, 161.0 / 255.0, 100.0 / 255.0];

const BAR_HEIGHT: f32 = 60.0;
const BAR_PADDING: f32 = 15.0;
const BAR_SEGMENTS: f32 = 3.0;
const BAR_SEGMENT_HEIGHT: f32 = (BAR_HEIGHT - 2.0) / BAR_SEGMENTS;


pub struct WalkRune {
	drawing: bool,
	sprite: Image,
	pos: Vec2,
	grid_pos: Vec2,
	timer: f32,

	offset: f32,
	float: f32,
	float_amp: f32,
	float_freq: f32,

	rot: f32,
	rot_amp: f32,
	rot_freq: f32,

}

impl WalkRune {
	fn new(ctx: &mut Context) -> Self {
		WalkRune {
			drawing: false,
			sprite: Image::from_path(ctx, "/walk_rune.png").unwrap(),
			pos: Vec2::ZERO,
			grid_pos: Vec2::ZERO,
			timer: 0.0,

			offset: -8.0,
			float: 0.0,
			float_amp: 1.7,
			float_freq: 3.0,

			rot: 0.0,
			rot_amp: 10.0,
			rot_freq: 2.0,
		}
	}

	fn update(self: &mut WalkRune, ctx: &mut Context, dt: &f32) -> GameResult {
		self.timer += dt;

		if let Some(rune_pos) = GridDrawer::mouse_pos_on_grid(ctx)? {
			self.drawing = true;

			let mut screen_rp = Vec2::ZERO;
			GridDrawer::grid_pos_to_screen(ctx, &rune_pos, &mut screen_rp)?;
			self.grid_pos = rune_pos;

			self.pos = screen_rp;
		}

		self.float = self.offset + sin(PI * self.timer * self.float_freq) * self.float_amp;
		self.rot = sin(PI * self.timer * self.rot_freq) * self.rot_amp;

		Ok(())
	}

	fn draw(self: &mut WalkRune, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		if !self.drawing {
			return Ok(());
		}

		let dp = DrawParam::default()
			.dest_rect(Rect::new(
				self.pos.x,
				self.pos.y + self.float * GlobConst::SCALE,
				GridDrawer::TILE_SIZE / self.sprite.width() as f32 * GlobConst::SCALE,
				GridDrawer::TILE_SIZE / self.sprite.height() as f32 * GlobConst::SCALE
			))
			.offset(vec2(0.5, 0.5))
			.rotation(self.rot * PI / 180.0);

		canvas.draw(&self.sprite, dp);

		Ok(())
	}

	fn on_possible_position(self: &mut WalkRune, possible_positions: &Vec<(i32, i32)>) -> bool {
		 possible_positions.contains(&(self.grid_pos.x as i32, self.grid_pos.y as i32))
	}
}


pub enum ValueType {
	Attack = 0,
	Armor = 1,
	Health = 2,
}

pub struct Player {
	pub pos: Vec2,		// position in the grid
	screen_pos: Vec2,	// position on screen
	size: Vec2,		// temporary: later sprite/image size
	rune_positions: Vec<(i32, i32)>,
	rune_sprite: Image,

	walk_rune: WalkRune,
	moving: bool,

	bars: [Image; 3],
	icons: Image,
	frame: Image,
	separator: Image,

	values: [f32; 3],
}

impl Player {
	
	pub fn new(ctx: &mut Context, x: u32, y: u32) -> Player {
		let mut other_bars = [[0u8; 256]; 3];

		// CALCULATE OTHER BARS WITH BAR_PXS ARRAY
		for idx in 0..3 {
			for pos in 0..5 {
				for z in 0..3 {
					other_bars[idx][pos * 4 + z] = (BAR_PXS[pos] * COLORS[idx][z] as f32) as u8;
				}
				other_bars[idx][pos * 4 + 3] = 255;
			}
		}

		let bar = Image::from_path(ctx, "/bar_gray.png").unwrap();

		let red_bar = Image::from_pixels(ctx, &other_bars[1], bar.format(), 5, 1);
		let green_bar = Image::from_pixels(ctx, &other_bars[0], bar.format(), 5, 1);
		let blue_bar = Image::from_pixels(ctx, &other_bars[2], bar.format(), 5, 1);

		let icons = Image::from_path(ctx, "/icons.png").unwrap();

		Player {
			pos: vec2(x as f32, y as f32),
			screen_pos: vec2(0.0, 0.0),
			size: vec2(20.0, 30.0),
			rune_positions: vec!(),
			rune_sprite: Image::from_path(ctx, "/rune.png").unwrap(),

			walk_rune: WalkRune::new(ctx),
			moving: false,

			bars: [red_bar, blue_bar, green_bar],
			icons: icons,
			frame: Image::from_path(ctx, "/frame.png").unwrap(),
			separator: Image::from_path(ctx, "/separator.png").unwrap(),
			
			values: [3.0, 2.0, 1.0],
		}
	}

	pub fn update(self: &mut Player, ctx: &mut Context, dt: &f32) -> GameResult {
		// convert grid position to screen position
		GridDrawer::grid_pos_to_screen(ctx, &self.pos, &mut self.screen_pos)?;
		
		// pivot at the feet
		self.screen_pos -= vec2(self.size.x * 0.5, self.size.y) * GlobConst::SCALE;

		if self.moving {
			self.walk_rune.update(ctx, dt)?;
			
			if ctx.mouse.button_just_pressed(MouseButton::Left) && self.walk_rune.on_possible_position(&self.rune_positions) {
				self.pos = self.walk_rune.grid_pos;
				self.moving = false;
				self.rune_positions.clear();
			}
		}

		Ok(())
	}

	pub fn draw(self: &mut Player, ctx: &mut Context, canvas: &mut Canvas, quad_mesh: &Mesh) -> GameResult {

		// draw rune positions, e.g. positions where the player can stop on
		for rp in &self.rune_positions {
			let mut screen_pos = Vec2::ZERO;
			GridDrawer::grid_pos_to_screen(ctx, &vec2(rp.0 as f32, rp.1 as f32), &mut screen_pos)?;
			let dp = DrawParam::default()
				.dest_rect(Rect::new(
					screen_pos.x - GridDrawer::TILE_SIZE as f32 * 0.5 * GlobConst::SCALE,
					screen_pos.y - GridDrawer::TILE_SIZE as f32 * 0.5 * GlobConst::SCALE,
					GridDrawer::TILE_SIZE as f32 / self.rune_sprite.width() as f32 * GlobConst::SCALE,
					GridDrawer::TILE_SIZE as f32 / self.rune_sprite.height() as f32 * GlobConst::SCALE
				));

			canvas.draw(&self.rune_sprite, dp);
		}

		// draw rune
		if self.moving {
			self.walk_rune.draw(ctx, canvas)?;
		}

		// maybe save player rect in the struct?
		let draw_param = DrawParam::default()
			.dest_rect(Rect::new(
				self.screen_pos.x,
				self.screen_pos.y,
				self.size.x / GlobConst::QUAD_SIZE * GlobConst::SCALE,
				self.size.y / GlobConst::QUAD_SIZE * GlobConst::SCALE
			));

		canvas.draw(quad_mesh, draw_param);
		
		self.draw_icons(ctx, canvas)?;

		Ok(())
	}

	fn draw_icons(self: &mut Player, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		let mut dp = DrawParam::default()
			.scale(GlobConst::SCALE_VECTOR);

		let (width, height) = ctx.gfx.drawable_size();
		let (width_half, height_half) = (width as f32 * 0.5, height as f32 * 0.5);

		for i in 0..3 {
			
			let height = height_half - (i as f32 - 1.0) * (BAR_HEIGHT + BAR_PADDING) * GlobConst::SCALE + BAR_HEIGHT * 0.5 * GlobConst::SCALE;
			let x = 5.0;

			let bar_pos = vec2((x + 3.0) * GlobConst::SCALE, height - BAR_HEIGHT * GlobConst::SCALE);

			dp = dp.dest(bar_pos)
				.src(Rect::new(0.0, 0.0, 1.0, 1.0));

			canvas.draw(&self.frame, dp);

			for j in 0..BAR_SEGMENTS as u32 {

				if j as f32 <= self.values[i] - 1.0 {
					dp = dp.dest(bar_pos + Vec2::ONE * GlobConst::SCALE + vec2(0.0, (BAR_SEGMENTS - j as f32 - 1.0) * BAR_SEGMENT_HEIGHT * GlobConst::SCALE))
						.scale(vec2(GlobConst::SCALE, (BAR_SEGMENT_HEIGHT - 1.0) * GlobConst::SCALE));

					canvas.draw(&self.bars[i as usize], dp);

				}

				if j < BAR_SEGMENTS as u32 - 1 {
					dp = dp.dest(bar_pos + Vec2::ONE * GlobConst::SCALE + vec2(0.0, (j as f32 + 1.0) * BAR_SEGMENT_HEIGHT * GlobConst::SCALE - GlobConst::SCALE))
						.scale(GlobConst::SCALE_VECTOR);

					canvas.draw(&self.separator, dp);
				}
			}

			dp = dp.dest(vec2(x * GlobConst::SCALE, height - 7.0 * GlobConst::SCALE))
				.scale(GlobConst::SCALE_VECTOR)
				.src(self.icons.uv_rect(i as u32 * 13, 0, 13, 13));

			canvas.draw(&self.icons, dp);
			
		}

		Ok(())
	}

	
	pub fn change_at(self: &mut Player, v_type: ValueType, amount: f32) -> GameResult {
		
		self.values[v_type as usize] += amount;

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

	pub fn move_player(self: &mut Player) {
		for neighbor in NEIGHBORS {
			let (x, y) = (neighbor.0 + self.pos.x as i32, neighbor.1 + self.pos.y as i32);
			if x < 0 || x >= GridDrawer::TILES_PER_ROW as i32 || y < 0 || y >= GridDrawer::TILES_PER_ROW as i32 {
				continue;
			}

			self.rune_positions.push((x, y));
		}
		self.moving = true;
	}
}
