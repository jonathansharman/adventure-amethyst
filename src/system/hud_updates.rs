use crate::{
	component::{
		Health,
		Hero,
	},
	resource::{Hud, SpriteSheets},
};

use amethyst::{
	derive::SystemDesc,
	ecs::{Entities, Join, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage},
	ui::{Anchor, UiImage, UiTransform},
};

/// Updates the HUD to display current information.
#[derive(SystemDesc)]
pub struct HudUpdates;

impl<'a> System<'a> for HudUpdates {
	type SystemData = (
		Entities<'a>,
		WriteExpect<'a, Hud>,
		ReadExpect<'a, SpriteSheets>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Health>,
		WriteStorage<'a, UiImage>,
		WriteStorage<'a, UiTransform>,
	);

	fn run(&mut self, (
		entities,
		mut hud,
		sprite_sheets,
		sto_hero,
		sto_health,
		mut sto_ui_image,
		mut sto_ui_transform,
	): Self::SystemData) {
		for (_hero, health) in (&sto_hero, &sto_health).join() {
			if hud.current_hearts > health.current() {
				// Delete excess heart images.
				for to_delete in health.current()..hud.current_hearts {
					entities.delete(hud.heart_images[to_delete as usize]).unwrap();
				}
				hud.current_hearts = health.current();
				hud.heart_images.truncate(health.current() as usize);
			} else if hud.current_hearts < health.current() {
				// Add heart images.
				for i in hud.current_hearts..health.current() {
					const HEARTS_MARGIN_TOP: f32 = 10.0;
					const HEARTS_MARGIN_LEFT: f32 = 10.0;
					const HEARTS_PADDING: f32 = 10.0;
					const HEARTS_WIDTH: f32 = 13.0;
					const HEARTS_HEIGHT: f32 = 12.0;
					const HEARTS_SCALE: f32 = 2.0;
					let heart_image = entities
						.build_entity()
						.with(
							UiImage::PartialTexture {
								tex: sprite_sheets.icons.clone(),
								left: 1.0 / 3.0,
								right: 2.0 / 3.0,
								bottom: 1.0,
								top: 0.0,
							},
							&mut sto_ui_image,
						)
						.with(
							UiTransform::new(
								"".to_string(),
								Anchor::TopLeft,
								Anchor::TopLeft,
								HEARTS_MARGIN_LEFT + (i as f32) * (HEARTS_PADDING + HEARTS_WIDTH * HEARTS_SCALE),
								-HEARTS_MARGIN_TOP,
								0.0,
								HEARTS_WIDTH * HEARTS_SCALE,
								HEARTS_HEIGHT * HEARTS_SCALE,
							),
							&mut sto_ui_transform,
						)
						.build();
					hud.heart_images.push(heart_image);
				}
				hud.current_hearts = health.current();
			}
		}
	}
}
