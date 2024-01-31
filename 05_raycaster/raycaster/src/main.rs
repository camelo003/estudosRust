use bracket_lib::prelude::*;
// use bracket_geometry::prelude::*;

const SCREEN_W: i32 = 80;
const SCREEN_H: i32 = 50;

const PLAYER_RADIUS: i32 = 1;
const DIR_LENGTH: f32 = 15.0;
const ROT_STEP: f32 = 0.1;

const TILE_SIZE: i32 = 5;

const TILES_W: i32 = SCREEN_W / TILE_SIZE;
const TILES_H: i32 = SCREEN_H / TILE_SIZE;

enum GameMode {
	ThreeD,
	TwoD,
}

struct Map {
	tiles: Vec<bool>,
}

impl Map {
	fn new() -> Self {
		let mut temp_tiles: Vec<bool> = vec![];
		for i in concat!("1111111111111111",
						 "1000000000010001",
						 "1000011000010001",
						 "1000001000010001",
						 "1000000000000001",
						 "1000100000000001",
						 "1000100001111001",
						 "1000100001000001",
						 "1000100000000001",
						 "1111111111111111").chars() {
			if i == '1' {
				temp_tiles.push(true);
			}else{
				temp_tiles.push(false);
			}
		}
		Self {
			tiles: temp_tiles,
		}
	}
	fn render (&self, ctx: &mut BTerm) {
		let mut counter = 0;
		for i in &self.tiles {
			let _x1 = (counter * TILE_SIZE) % SCREEN_W;
			let _y1 = counter / TILES_W * TILE_SIZE;
			let _x2 = _x1 + TILE_SIZE;
			let _y2 = _y1 + TILE_SIZE;
			if *i {
				let r: Rect = Rect {x1: _x1, y1: _y1, x2: _x2, y2: _y2};
				r.for_each(|p: Point| ctx.set(p.x, p.y, GREY, BLACK, '+'));
			}else{
				let l1: Bresenham = Bresenham::new(Point {x: _x1, y: _y1},
												   Point {x: _x2, y: _y1});
				let l2: Bresenham = Bresenham::new(Point {x: _x1, y: _y1},
												   Point {x: _x1, y: _y2});
				l1.for_each(|p: Point| ctx.set(p.x, p.y, GREY, BLACK, '+'));
				l2.for_each(|p: Point| ctx.set(p.x, p.y, GREY, BLACK, '+'));
			}
			counter = counter + 1;
		}
	}
}

struct State {
	mode: GameMode,
	player: Player,
	map: Map,
}

impl State {
	fn new() -> Self {
		Self {
			mode: GameMode::TwoD,
			player: Player::new(),
			map: Map::new(),
		}
	}
	fn three_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print(1, 1, "Hellow 3D World!");
	}
	fn two_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print(1, 1, "Hellow 2D World!");
		self.map.render(ctx);
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
		let p = f32::tan(0.436332) * DIR_LENGTH;
		Self {
			camera: [10.0, 10.0],
			direction: [DIR_LENGTH, 0.0],
			plane: [0.0, p],
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
					);
					self.plane = Self::rot(
						self.plane,
						-ROT_STEP
					);
				},
				VirtualKeyCode::D => {
					self.direction = Self::rot(
						self.direction,
						ROT_STEP
					);
					self.plane = Self::rot(
						self.plane,
						ROT_STEP
					);
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
		let pln = Point::new(
			(self.plane[0] / 2.0).round() as i32,
			(self.plane[1] / 2.0).round() as i32
		);
		for point in Bresenham::new(pos, pos + dir + pln) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
		for point in Bresenham::new(pos, pos + dir - pln) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
		for point in Bresenham::new(pos + dir + pln, pos + dir - pln) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
		for point in BresenhamCircle::new(pos, PLAYER_RADIUS) {
			ctx.set(point.x, point.y, GREEN, BLACK, '*');
		}
		ctx.set(pos.x, pos.y, WHITE, BLACK, 'P');
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
