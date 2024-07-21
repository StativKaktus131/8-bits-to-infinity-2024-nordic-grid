use ggez::{
	graphics::*,
	event::{self, EventHandler},
	conf::{Conf, Backend, WindowSetup, WindowMode, FullscreenType},
	glam::*,
	input::keyboard::{KeyCode, KeyMods, KeyInput},
	Context, ContextBuilder, GameResult
};

use crate::global_constants as GlobConst;
use crate::math::lerp;

use std::f32::consts::PI as PI;


const INTER: f32 = 10.0;
const MOVE_SPEED: f32 = 40.0;
const ROT_SPEED: f32 = 30.0;

const CARD_WIDTH: u32 = 31;
const CARD_HEIGHT: u32 = 45;



#[derive(Copy, Clone)]
pub enum CardType {
	Move = 0,
	Armor = 1,
	Health = 2,
	Key = 3
}


pub struct Card {
	//transform variables
	pos: Vec2,
	card_level: f32,
	target_pos: Vec2,
	rotation: f32,
	pub target_rotation: f32,

	// images
	sprite: Image,
	sprite_rect: Rect,
	highlighted_border: Image,
	shade: Image,

	// selection things
	pub selected: bool,
	pub pos_rel_to_selected: f32,

	// misc
	pub card_type: CardType,
	drawing_shade: bool,
}

impl Card {
	pub fn new(ctx: &mut Context, card_type: CardType) -> Card {
		let (width, height) = ctx.gfx.drawable_size();
		let (width_half, height_half) = (width * 1.5, height * 0.5);

		let sprite = Image::from_path(ctx, "/cards.png").unwrap();
		Card {
			pos: vec2(0.0, height - CARD_HEIGHT as f32 * GlobConst::SCALE + 35.0),
			card_level: height - CARD_HEIGHT as f32 * GlobConst::SCALE + 35.0,
			target_pos: Vec2::ZERO,
			rotation: 0.0,
			target_rotation: 0.0,

			sprite: sprite.clone(),
			sprite_rect: sprite.uv_rect(CARD_WIDTH *card_type as u32, 0, CARD_WIDTH, CARD_HEIGHT),
			highlighted_border: Image::from_path(ctx, "/highlighted_border.png").unwrap(),
			shade: Image::from_path(ctx, "/card_shade.png").unwrap(),

			selected: false,
			pos_rel_to_selected: 0.0,

			card_type: card_type,
			drawing_shade: false,
		}
	}

	pub fn update(self: &mut Card, ctx: &mut Context, amount_of_cards: u8, index: u8, dt: &f32) -> GameResult {
		

		let (width, height) = ctx.gfx.drawable_size();
		let (width_half, height_half) = (width * 0.5, height * 0.5);

		let n = amount_of_cards as f32;
		let w = CARD_WIDTH as f32;

		self.target_pos.x = width_half + (n / 2.0 - index as f32 - 0.5) * (w - INTER) * GlobConst::SCALE - w * 0.5 * GlobConst::SCALE + self.pos_rel_to_selected;
		self.target_pos.y = self.card_level - if self.selected { 36.0 } else { 0.0 };

		self.pos = self.pos.lerp(self.target_pos, MOVE_SPEED * dt);
		self.rotation = lerp(&self.rotation, &self.target_rotation, &(ROT_SPEED * dt));

		self.drawing_shade = index != 0;

		Ok(())
	}

	pub fn draw(self: &mut Card, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		
		let mut draw_param = DrawParam::default()
			.src(self.sprite_rect)
			.dest(self.pos)
			.scale(GlobConst::SCALE_VECTOR)
			.rotation(self.rotation);
		
		canvas.draw(&self.sprite, draw_param);

		if self.drawing_shade {
			let lvl = ((self.card_level - self.pos.y) / GlobConst::SCALE * 1.35) as u32;
			let dp = DrawParam::default()
				.src(self.shade.uv_rect(0, lvl, self.shade.width(), CARD_HEIGHT - lvl))
				.dest(vec2(self.pos.x - self.rotation * 165.0 / PI, self.pos.y + lvl as f32 * GlobConst::SCALE))
				.scale(GlobConst::SCALE_VECTOR)
				.rotation(self.rotation);

			canvas.draw(&self.shade, dp);
		}

		draw_param.src = Rect::new(0.0, 0.0, 1.0, 1.0);

		if self.selected {
			canvas.draw(&self.highlighted_border, draw_param);
		}
		
		Ok(())
	}

	pub fn mouse_over(self: &mut Card, x: f32, y: f32) -> bool {
		x >= self.pos.x && x <= self.pos.x + CARD_WIDTH as f32 * GlobConst::SCALE && y >= self.pos.y && y <= self.pos.y + CARD_HEIGHT as f32 * GlobConst::SCALE
	}
}
