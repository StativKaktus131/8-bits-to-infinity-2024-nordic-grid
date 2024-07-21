use serde::Deserialize;

use ggez::{
	glam::*,
	Context
};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::card::*;
use crate::player::*;
use crate::explainer::*;
use crate::grid_drawer as GridDrawer;


#[derive(Deserialize, Debug)]
pub struct Level {
	map: Vec<Vec<u8>>,
	cards: Vec<String>,
	cards_on_hand: u8,
	explain_on_startup: String,
}

pub fn load_level<P: AsRef<Path>>(ctx: &mut Context, path: P, cards: &mut Vec<Card>, player: &mut Player, explainer: &mut Explainer) -> Result<Level, Box<dyn Error>> {
	
	let current_dir = std::env::current_dir().unwrap();

	let file = File::open(current_dir.join(path))?;
	let reader = BufReader::new(file);

	let level: Level = serde_json::from_reader(reader)?;


	// instantiate all the cards
	for card in &level.cards {
		let card_type = match card.to_uppercase().as_ref() {
			"HEALTH" => Some(CardType::Health),
			"ARMOR" => Some(CardType::Armor),
			"MOVE" => Some(CardType::Move),
			"KEY" => Some(CardType::Key),
			_ => None
		};

		if let Some(ct) = card_type {
			cards.push(Card::new(ctx, ct));
		}

	}

	// add tilemap thing
	for (y, row) in level.map.iter().enumerate() {
		for (x, element) in row.iter().enumerate() {
			match element {
				1 => {
					player.pos = vec2(x as f32, y as f32);
					player.target_pos = vec2(x as f32, y as f32);
				},
				2 => GridDrawer::set_state_at(x as usize, y as usize, GridDrawer::State::Tree),
				3 => GridDrawer::set_state_at(x as usize, y as usize, GridDrawer::State::Chest),
				_ => (),
			}
		}
	}
	
	let ct = match level.explain_on_startup.to_uppercase().as_ref() {
		"MOVE" => Some(CardType::Move),
		"KEY" => Some(CardType::Key),
		_ => None
	};

	if let Some(t) = ct {
		explainer.explain(t);
	}

	Ok(level)
}
