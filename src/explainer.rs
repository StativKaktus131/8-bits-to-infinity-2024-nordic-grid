use ggez::{
	graphics::*,
	event::{self, EventHandler},
	conf::{Conf, Backend, WindowSetup, WindowMode, FullscreenType},
	glam::*,
	mint::Point2,
	input::keyboard::{KeyCode, KeyMods, KeyInput},
	input::mouse::{MouseContext, MouseButton},
	Context, ContextBuilder, GameResult, GameError
};

use crate::card::*;
use crate::global_constants as GlobConst;


const DIFF_CARD_TYPES: usize = 4;
const POSITION: Vec2 = vec2(1350.0, 100.0);


pub struct Explainer {
	explained: [bool; DIFF_CARD_TYPES],
	drawing: bool,

	background_mesh: Mesh,
	title_images: [Image; DIFF_CARD_TYPES],
	explanations: [Text; DIFF_CARD_TYPES],
	explain_idx: usize,
	close_text: Text,
}


impl Explainer {
	pub fn new(ctx: &mut Context) -> Explainer {
		Explainer {
			explained: [false; DIFF_CARD_TYPES],
			drawing: false,

			background_mesh: Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, 400.0, 480.0), Color::new(0.0, 0.0, 0.0, 0.4)).unwrap(),
			title_images: [Image::from_path(ctx, "/titles/key_title.png").unwrap(), Image::from_path(ctx, "/titles/key_title.png").unwrap(), Image::from_path(ctx, "/titles/key_title.png").unwrap(), Image::from_path(ctx, "/titles/key_title.png").unwrap()],
			explanations: [
				Text::new("The MOVE card moves you to another field in your range. Use it to get closer to a chest."),
				Text::new("The KEY card opens any chest touching the player"),
				Text::new("The KEY card opens any chest touching the player"),
				Text::new("The KEY card opens any chest touching the\nplayer. You will advance to the next level."),
			],
			explain_idx: 0,
			close_text: Text::new("PRESS 'C' TO CLOSE"),
		}
	}

	pub fn explain(self: &mut Explainer, card_type: CardType) {
		if self.explained[card_type as usize] {
			return;
		}

		self.explained[card_type as usize] = true;

		self.explain_idx = card_type as usize;
		self.drawing = true;
	}

	pub fn draw(self: &mut Explainer, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		
		if !self.drawing {
			return Ok(());
		}

		let mut dp = DrawParam::default()
			.dest(POSITION);

		canvas.draw(&self.background_mesh, dp);

		dp = dp
			.dest(POSITION + vec2(20.0, 20.0))
			.scale(GlobConst::SCALE_VECTOR);
		
		canvas.draw(&self.title_images[self.explain_idx], dp);

		dp = dp.dest(POSITION + vec2(20.0, 150.0)).scale(Vec2::ONE);

		canvas.draw(&self.explanations[self.explain_idx], dp);

		dp = dp.dest(POSITION + vec2(20.0, 440.0));

		canvas.draw(&self.close_text, dp);

		Ok(())
	}
}


