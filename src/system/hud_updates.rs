use crate::{
	component::{
		Health,
		Hero,
	},
	resource::{Hud, Textures},
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
		ReadExpect<'a, Textures>,
		ReadStorage<'a, Hero>,
		ReadStorage<'a, Health>,
		WriteStorage<'a, UiImage>,
		WriteStorage<'a, UiTransform>,
	);

	fn run(&mut self, (
		entities,
		mut hud,
		textures,
		sto_hero,
		sto_health,
		mut sto_ui_image,
		mut sto_ui_transform,
	): Self::SystemData) {
		for (_hero, health) in (&sto_hero, &sto_health).join() {
			if hud.max_hearts > health.max() {
				// Delete excess heart images.
				for to_delete in health.max()..hud.max_hearts {
					entities.delete(hud.heart_images[to_delete as usize]).unwrap();
				}
				hud.heart_images.truncate(health.max() as usize);
			} else if hud.max_hearts < health.max() {
				// Add heart images.
				for i in hud.max_hearts..health.max() {
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
								tex: textures.hearts.clone(),
								left: 0.0,
								right: 1.0 / 3.0,
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
			}
			hud.max_hearts = health.max();
			// Update heart image sprites.
			for i in 0..health.current() as usize {
				if let UiImage::PartialTexture { left, right, .. } = sto_ui_image.get_mut(hud.heart_images[i]).unwrap() {
					*left = 1.0 / 3.0;
					*right = 2.0 / 3.0;
				}
			}
			for i in health.current() as usize..health.max() as usize {
				if let UiImage::PartialTexture { left, right, .. } = sto_ui_image.get_mut(hud.heart_images[i]).unwrap() {
					*left = 0.0;
					*right = 1.0 / 3.0;
				}
			}
		}
	}
}
