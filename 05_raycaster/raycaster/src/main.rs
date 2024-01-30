use bracket_lib::prelude::*;
use bracket_geometry::prelude::*;

const PLAYER_RADIUS: i32 = 2;
const DIR_LENGTH: f32 = 4.0;
const ROT_STEP: f32 = 0.2;

enum GameMode {
	ThreeD,
	TwoD,
}

struct State {
	mode: GameMode,
	player: Player,
}

impl State {
	fn new() -> Self {
		Self {
			mode: GameMode::TwoD,
			player: Player::new(),
		}
	}
	fn three_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print(1, 1, "Hellow 3D World!");
	}
	fn two_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print(1, 1, "Hellow 2D World!");
		self.player.update(ctx);
		self.player.render(ctx);
	}
}

struct Player {
	camera: [f32; 2],
	direction: [f32; 2],
	plane: [f32; 2],
}

impl Player {
	fn new() -> Self {
		Self {
			camera: [10.0, 10.0],
			direction: [DIR_LENGTH, 0.0],
			plane: [0.0, 1.0],
		}
	}
	fn normalize(v: [f32; 2]) -> [f32; 2] {
		let [x, y] = v;
		let mag = f32::sqrt(x.powi(2) + y.powi(2));
		[x / mag, y / mag]
	}
	fn sum(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
		[a[0] + b [0], a[1] + b[1]]
	}
	fn sub(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
		Self::sum(a, [-b[0], -b[1]])
	}
	fn rot(v: [f32; 2], r: f32) -> [f32; 2] {
		[
			r.cos() * v[0] - r.sin() * v[1],
			r.sin() * v[0] + r.cos() * v[1]
		]
	}
	fn update(&mut self, ctx: &mut BTerm) {
		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::W => {
					self.camera = Self::sum(
						self.camera,
						Self::normalize(self.direction)
					)
				},
				VirtualKeyCode::S => {
					self.camera = Self::sub(
						self.camera,
						Self::normalize(self.direction)
					)
				},
				VirtualKeyCode::A => {
					self.direction = Self::rot(
						self.direction,
						-ROT_STEP
					)
				},
				VirtualKeyCode::D => {
					self.direction = Self::rot(
						self.direction,
						ROT_STEP
					)
				},
				_ => {},
			}
		}
	}
	fn render(&self, ctx: &mut BTerm) {
		let pos = Point::new(
			self.camera[0].round() as i32,
			self.camera[1].round() as i32
		);
		let dir =  Point::new(
			self.direction[0].round() as i32,
			self.direction[1].round() as i32
		);
		ctx.set(pos.x, pos.y, WHITE, BLACK, 'P');
		for point in BresenhamCircle::new(pos, PLAYER_RADIUS) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
		for point in Bresenham::new(pos, pos + dir) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
	}
}

impl GameState for State {
	fn tick (&mut self, ctx: &mut BTerm) {
		match self.mode {
			GameMode::ThreeD => self.three_d(ctx),
			GameMode::TwoD => self.two_d(ctx),
		}
	}
}

fn main() -> BError {
	let context = BTermBuilder::simple80x50()
		.with_title("Raycaster").
		build()?;
	main_loop(context, State::new())
}
