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
		.with(system::HeroControl, "hero_control", &["input_system"])
		.with(system::EnemyControl, "enemy_control", &[])
		.with(system::Knockback, "knockback", &[])
		.with_barrier()
		.with(system::Motion, "motion", &[])
		.with_barrier()
		.with(system::Travel, "travel", &[])
		.with_barrier()
		.with(system::StaticCollisionDetection, "static_collision_detection", &[])
		.with_barrier()
		.with(system::DynamicCollisionDetection, "dynamic_collision_detection", &[])
		.with_barrier()
		.with(system::Death, "death", &[])
		.with_barrier()
		.with(system::SwordAttackUpdates, "sword_attack_updates", &[])
		.with_barrier()
		.with(system::Animation, "animation", &[])
		.with(system::CameraControl, "camera_control", &[])
		.with(system::HudUpdates, "hud_updates", &[])
	};

	Application::new(resources, state::Playing, game_data)?.run();

	Ok(())
}
