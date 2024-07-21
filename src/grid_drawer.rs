use ggez::{
	graphics::*,
	glam::*,
	Context, GameResult, GameError
};

use std::sync::{OnceLock, Mutex};

use crate::global_constants as GlobConst;
use crate::player::*;


//consts
const TILE_FIELD_SIZE: f32 = 175.0;
const TILE_FIELD_Y: f32 = 25.0;
pub const TILES_PER_ROW: f32 = 7.0;				// pub bc is used by the player
pub const TILE_SIZE: f32 = TILE_FIELD_SIZE / TILES_PER_ROW;


#[derive(PartialEq, Clone, Copy)]
pub enum State {
	Empty = 0,
	Tree = 1,
	Chest = 2,
}

pub fn states() -> &'static Mutex<[[State; TILES_PER_ROW as usize]; TILES_PER_ROW as usize]> {
	static STATES: OnceLock<Mutex<[[State; TILES_PER_ROW as usize]; TILES_PER_ROW as usize]>> = OnceLock::new();
	STATES.get_or_init(|| Mutex::new([[State::Empty; TILES_PER_ROW as usize]; TILES_PER_ROW as usize]))
}


pub fn set_state_at(x: usize, y: usize, state: State) {
	states().lock().unwrap()[x][y] = state;
}


pub fn draw_grid(ctx: &mut Context, canvas: &mut Canvas, images: &Vec<Image>) -> GameResult {
	
	let (width, height) = ctx.gfx.drawable_size();
	let (width_half, height_half) = (width * 0.5, height * 0.5);
	
	for x in 0..TILES_PER_ROW as u8 {
		for y in 0..TILES_PER_ROW as u8 {

			let tile_pos = vec2(
					width_half - TILE_FIELD_SIZE * 0.5 * GlobConst::SCALE + x as f32 * TILE_SIZE * GlobConst::SCALE,
					TILE_FIELD_Y * GlobConst::SCALE + y as f32 * TILE_SIZE * GlobConst::SCALE
				);
			let mut dp = DrawParam::default()
				.dest_rect(Rect::new(
					// generally: TILE_FIELD_START + x + y
					tile_pos.x,
					tile_pos.y,

					// width again because 30x30
					// i think dest_rect varies the pixel rect ??
					TILE_SIZE / images[0].width() as f32 * GlobConst::SCALE,
					TILE_SIZE / images[0].width() as f32 * GlobConst::SCALE
				));

			canvas.draw(&images[0], dp);
			
			let state: State = states().lock().unwrap()[x as usize][y as usize];

			match state {
				State::Tree => {
					draw_figurine(ctx, canvas, &images[1], &(tile_pos.x, tile_pos.y));
				},
				State::Chest => {
					draw_figurine(ctx, canvas, &images[2], &(tile_pos.x, tile_pos.y));
				}
				State::Empty => (),
			}
		}
	}

	Ok(())
}

pub fn draw_figurine(ctx: &mut Context, canvas: &mut Canvas, img: &Image, pos: &(f32, f32)) {
	
	let (w, h) = (img.width(), img.height());
	let dp = DrawParam::default().dest_rect(Rect::new(
		pos.0 + TILE_SIZE * 0.5 * GlobConst::SCALE - w as f32 * 0.5 * GlobConst::SCALE,
		pos.1 + TILE_SIZE * 0.5 * GlobConst::SCALE - h as f32 * 0.75 * GlobConst::SCALE,
		GlobConst::SCALE, GlobConst::SCALE
	))
		.z((pos.1 + TILE_SIZE * 0.5 * GlobConst::SCALE - h as f32 * 0.75 * GlobConst::SCALE) as i32);

	canvas.draw(img, dp);
}


pub fn get_state(x: usize, y: usize) -> State {
	assert!(x < TILES_PER_ROW as usize && y < TILES_PER_ROW as usize);

	states().lock().unwrap()[x][y]
}


pub fn grid_pos_to_screen(ctx: &mut Context, pos: &Vec2, screen_pos: &mut Vec2) -> GameResult {
	assert!(pos.x >= 0.0 && pos.x < TILES_PER_ROW && pos.y >= 0.0 && pos.y < TILES_PER_ROW);
	
	let (width, height) = ctx.gfx.drawable_size();

	screen_pos.x = width * 0.5 - TILE_FIELD_SIZE * 0.5 * GlobConst::SCALE + pos.x * TILE_SIZE * GlobConst::SCALE + TILE_SIZE * 0.5 * GlobConst::SCALE;
	screen_pos.y = TILE_FIELD_Y * GlobConst::SCALE + pos.y * TILE_SIZE * GlobConst::SCALE + TILE_SIZE * 0.5 * GlobConst::SCALE;

	Ok(())
}

pub fn mouse_pos_on_grid(ctx: &mut Context) -> GameResult<Option<Vec2>> {
	let mp = ctx.mouse.position();
	let (width, height) = ctx.gfx.drawable_size();
	let (width_half, height_half) = (width * 0.5, height * 0.5);

	// check if not over tilemap
	if mp.x < width_half - TILE_FIELD_SIZE * 0.5 * GlobConst::SCALE ||
		mp.x > width_half + TILE_FIELD_SIZE * 0.5 * GlobConst::SCALE ||
		mp.y < TILE_FIELD_Y * GlobConst::SCALE ||
		mp.y > TILE_FIELD_Y * GlobConst::SCALE + TILE_SIZE * TILES_PER_ROW * GlobConst::SCALE {
		
		return Ok(None);
	}

	let pos_on_grid = (vec2(mp.x, mp.y) - vec2(width_half - TILE_FIELD_SIZE as f32 * 0.5 * GlobConst::SCALE, TILE_FIELD_Y * GlobConst::SCALE)) / GlobConst::SCALE / TILE_SIZE - vec2(0.5, 0.5);

	Ok(Some(pos_on_grid.round().clamp(Vec2::ZERO, vec2(TILES_PER_ROW as f32 - 1.0, TILES_PER_ROW as f32 - 1.0))))
}
