// !!! REMOVE FOR RELEASE !!!
#![allow(unused_imports, unused_variables)]


// imports
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
use std::f32::consts::PI as PI;

extern crate sdl2;
use sdl2::mouse::Cursor;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

// module declarations
pub mod global_constants;
pub mod grid_drawer;
pub mod player;
pub mod card;
pub mod math;
pub mod level_manager;
pub mod explainer;

// module imports
use global_constants as GlobConst;
use grid_drawer as GridDrawer;
use level_manager as LevelManager;
use player::*;
use card::*;
use explainer::*;


const TWO_PI: f32 = 2.0 * PI;



// GAME STATE STRUCT
struct Game {
	quad_mesh: Mesh,		// temporary
	player: Player,
	cards: Vec<Card>,
	explainer: Explainer,
	tile_images: Vec<Image>,	// make a struct in grid drawer?
}

impl Game {
	pub fn new(ctx: &mut Context) -> Game {
		let player = Player::new(ctx, 0, 0);
		Game {
			quad_mesh: Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, GlobConst::QUAD_SIZE, GlobConst::QUAD_SIZE), Color::WHITE).unwrap(),
			player: player,
			cards: Vec::<Card>::new(),
			explainer: Explainer::new(ctx),
			tile_images: vec!(
				Image::from_path(ctx, "/tile.png").unwrap(),
				Image::from_path(ctx, "/tree.png").unwrap(),
				Image::from_path(ctx, "/chest.png").unwrap()
			),
		}
	}
}

impl EventHandler for Game {
	fn update(self: &mut Game, ctx: &mut Context) -> GameResult {

		let dt = ctx.time.delta().as_secs_f32();

		self.player.update(ctx, &dt)?;

		let len = self.cards.len() as u8;
		let mp = ctx.mouse.position();
		let mut selected_i: i16 = -1;

		// highlight card
		for (i, card) in self.cards.iter_mut().enumerate() {
			card.update(ctx, len, i as u8, &dt)?;
			
			card.target_rotation = 0.0;
			card.selected = false;
			if card.mouse_over(mp.x, mp.y) {
				selected_i = i as i16;
			}
		}
		
		// else selected_i is -1
		if selected_i >= 0 {
			self.cards[selected_i as usize].selected = true;
			self.cards[selected_i as usize].pos_rel_to_selected = 0.0;
			self.cards[selected_i as usize].target_rotation = TWO_PI / -64.0;
			
			if ctx.mouse.button_just_pressed(MouseButton::Left) {
				self.use_card_at(selected_i as usize)?;
			}

			for (i, card) in self.cards.iter_mut().enumerate() {
				if !card.selected {
					card.pos_rel_to_selected = if selected_i < i as i16 { -11.0 } else { 11.0 }; // push away from selected card
				}
			}
		} else {
			for card in self.cards.iter_mut() {
				card.pos_rel_to_selected = 0.0;
			}
		}

		Ok(())
	}

	fn draw(self: &mut Game, ctx: &mut Context) -> GameResult {

		// get canvas, disable filtering, and dimensions
		let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 45));
		canvas.set_sampler(Sampler::nearest_clamp());
		let (screen_w, screen_h) = ctx.gfx.size();
		
		// draw grid with (temporary) quad mesh
		GridDrawer::draw_grid(ctx, &mut canvas, &self.tile_images)?;

		// draw player
		self.player.draw(ctx, &mut canvas, &self.quad_mesh)?;

		// draw cards
		for card in &mut self.cards {
			card.draw(ctx, &mut canvas)?;
		}

		self.explainer.draw(ctx, &mut canvas)?;

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

impl Game {
	fn use_card_at(self: &mut Game, idx: usize) -> GameResult {
		let card = &self.cards[idx];
		
		match &card.card_type {
			&CardType::Move => {
				self.player.move_player();
			},
			&CardType::Health => {
				self.player.change_at(ValueType::Health, 1.0)?;
			},
			&CardType::Armor => {
				self.player.change_at(ValueType::Armor, 1.0)?;
			},
			&CardType::Key => {
				self.player.use_key();
			},
		}

		self.cards.remove(idx);

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

	let mut game = Game::new(&mut ctx);

	let lvl = LevelManager::load_level(&mut ctx, "resources/levels/level1.json", &mut game.cards, &mut game.player, &mut game.explainer).unwrap();

	// println!("{:#?}", lvl);

	// let mut binding = Image::from_path(&mut ctx, "/glove_big.png").unwrap().to_pixels(&ctx).unwrap();
	// let glove = Surface::from_data(&mut binding, 16, 16, 0, PixelFormatEnum::RGBA32).unwrap();
	// let cursor = Cursor::from_surface(&glove, 8, 2).unwrap();

	// cursor.set();

	event::run(ctx, event_loop, game);
}
