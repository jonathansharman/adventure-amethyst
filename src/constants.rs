
/// The size of a tile in the world.
pub const TILE_SIZE: f32 = 40.0;

/// The base maximum health of the hero.
pub const HERO_BASE_HEALTH: i32 = 20;
/// The base maximum health of enemies.
pub const ENEMY_BASE_HEALTH: i32 = 3;

/// The shortest half-extent of a sword thrust's collider.
pub const THRUST_ATTACK_HALF_WIDTH: f32 = 6.0;
/// The longest half-extent of a sword thrust's collider.
pub const THRUST_ATTACK_HALF_LENGTH: f32 = 20.0;
/// The radius of a sword slash attack.
pub const SLASH_ATTACK_RADIUS: f32 = 40.0;
/// The shortest half-extent of a shield's collider.
pub const SHIELD_HALF_WIDTH: f32 = 6.0;
/// The longest half-extent of a shield's collider.
pub const SHIELD_HALF_LENGTH: f32 = 20.0;
/// The shortest half-extent of an arrow attack's collider.
pub const ARROW_ATTACK_HALF_WIDTH: f32 = 6.0;
/// The longest half-extent of an arrow attack's collider.
pub const ARROW_ATTACK_HALF_LENGTH: f32 = 20.0;
/// The minimum number of frames an archer AI will wait before shooting an arrow.
pub const MIN_ARROW_DELAY: u32 = 150;
/// The maximum number of frames an archer AI will wait before shooting an arrow.
pub const MAX_ARROW_DELAY: u32 = 300;
/// The speed of a launched arrow.
pub const ARROW_SPEED: f32 = 10.0;

/// The speed at which a character is knocked away when hit.
pub const KNOCKBACK_SPEED: f32 = 12.5;
/// The number of frames knockback lasts.
pub const KNOCKBACK_FRAMES: u32 = 5;

/// The number of frames a character is invulnerable after being damaged.
pub const INVULNERABLE_FRAMES: u32 = 60;
/// The period of the blinking animation during invulnerability, in frames.
pub const INVULNERABLE_BLINK_PERIOD: u32 = 2;
/// How bright a character should be during the dim part of the invulnerability blinking animation.
pub const INVULNERABLE_BLINK_LUMINANCE: f32 = 0.75;

/// The width of a heart pickup in the world.
pub const HEART_WIDTH: f32 = 26.0;
/// The height of a heart pickup in the world.
pub const HEART_HEIGHT: f32 = 24.0;
