use amethyst::{
	core::transform::TransformBundle,
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	ui::{RenderUi, UiBundle},
	utils::application_root_dir,
};

mod component;
mod constants;
mod input_bindings;
mod resource;
mod state;
mod system;

use system::*;

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?;
	let resources = app_root.join("assets");
	let display_config = app_root.join("config/display_config.ron");
	let key_bindings_path = app_root.join("config/input.ron");

	let game_data = { GameDataBuilder::default()
		// Transform
		.with_bundle(TransformBundle::new())?
		// Input
		.with_bundle(InputBundle::<input_bindings::InputBindings>::new()
			.with_bindings_from_file(&key_bindings_path)?
		)?
		// UI
		.with_bundle(UiBundle::<StringBindings>::new())?
		// Rendering
		.with_bundle(RenderingBundle::<DefaultBackend>::new()
			.with_plugin(RenderToWindow::from_config_path(display_config)?
				.with_clear([0.0, 0.0, 0.0, 1.0]),
			)
			.with_plugin(RenderUi::default())
			.with_plugin(RenderFlat2D::default()),
		)?
		// Systems
		.with(HeroControl::new(), "hero_control", &["input_system"])
		.with(EnemyControl, "enemy_control", &[])
		.with(Knockback, "knockback", &[])
		.with_barrier()
		.with(Motion, "motion", &[])
		.with_barrier()
		.with(Travel, "travel", &[])
		.with_barrier()
		.with(StaticCollisionDetection, "static_collision_detection", &[])
		.with_barrier()
		.with(DynamicCollisionDetection, "dynamic_collision_detection", &[])
		.with_barrier()
		.with(Death, "death", &[])
		.with_barrier()
		.with(SwordAttackUpdates, "sword_attack_updates", &[])
		.with_barrier()
		.with(Animation, "animation", &[])
		.with(CameraControl, "camera_control", &[])
		.with(HudUpdates, "hud_updates", &[])
	};

	Application::new(resources, state::Playing, game_data)?.run();

	Ok(())
}
