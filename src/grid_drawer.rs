use ggez::{
	graphics::*,
	glam::*,
	Context, GameResult, GameError
};

use crate::global_constants as GlobConst;
use crate::player::*;


//consts
const TILE_FIELD_SIZE: f32 = 175.0;
const TILE_FIELD_Y: f32 = 25.0;
pub const TILES_PER_ROW: f32 = 7.0;				// pub bc is used by the player
pub const TILE_SIZE: f32 = TILE_FIELD_SIZE / TILES_PER_ROW; 




pub fn draw_grid(ctx: &mut Context, canvas: &mut Canvas, tile_image: &Image) -> GameResult {
	
	let (width, height) = ctx.gfx.drawable_size();
	let (width_half, height_half) = (width * 0.5, height * 0.5);
	
	for x in 0..TILES_PER_ROW as u8 {
		for y in 0..TILES_PER_ROW as u8 {
			let dp = DrawParam::default()
				.dest_rect(Rect::new(
					// generally: TILE_FIELD_START + x + y
					width_half - TILE_FIELD_SIZE * 0.5 * GlobConst::SCALE + x as f32 * TILE_SIZE * GlobConst::SCALE,
					TILE_FIELD_Y * GlobConst::SCALE + y as f32 * TILE_SIZE * GlobConst::SCALE,
					
					// width again because 30x30
					// i think dest_rect varies the pixel rect ??
					TILE_SIZE / tile_image.width() as f32 * GlobConst::SCALE,
					TILE_SIZE / tile_image.width() as f32 * GlobConst::SCALE
				));

			canvas.draw(tile_image, dp);
		}
	}

	Ok(())
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
