use bracket_lib::prelude::*;
// use bracket_geometry::prelude::*;

const SCREEN_W: i32 = 80;
const SCREEN_H: i32 = 50;

const DIAGONAL: f32 = 94.3398113; // FIXME hardcoded screen hypotenuse: bad!

const PLAYER_RADIUS: i32 = 1;

const DIR_LENGTH: f32 = 1.0;
const ROT_STEP: f32 = 0.1;

const TILE_SIZE: i32 = 5;

const TILES_W: i32 = SCREEN_W / TILE_SIZE;
const TILES_H: i32 = SCREEN_H / TILE_SIZE;

enum GameMode {
	ThreeD,
	TwoD,
}

enum HorizontalDirection{
	Right,
	Left,
}

enum VerticalDirection{
	Up,
	Down,
}

struct Ray {
	origin: Vec<f32>,
	plane: Vec<f32>,
	h_dir: HorizontalDirection,
	v_dir: VerticalDirection,
}

impl Ray{
	fn new(origin: [f32; 2], plane: [f32; 2]) -> Self {
		let h_dir: HorizontalDirection;
		let v_dir: VerticalDirection;

		if origin[0] < plane[0] {
			h_dir = HorizontalDirection::Right;
		} else {
			h_dir = HorizontalDirection::Left;
		}
		if origin[1] < plane[1] {
			v_dir = VerticalDirection::Down;
		} else {
			v_dir = VerticalDirection::Up;
		}
		Self {
			origin: origin.to_vec(),
			plane: plane.to_vec(),
			h_dir,
			v_dir,
		}
	}
	fn check_column_collision(&self, map: &Map) -> (f32, Vec<f32>) {
		// 0. checa se oponto no plano não esta
		//    de-ntro da parede.
		// if map.tile_from_vec(&self.plane) {
		// if map.check_column_tile(&self, self.plane.clone()){
		//	println!("return 0.0 because plane is inside windows!");
		//	return (0.0, self.plane.clone());
		//}

		// 1. montar o triangulo
		/*
					  c
					 /|
			  hypo  / | height
			tenuse /  | side
				  /   |
				a ----- b
			   base  side
		 */

		let dx = f32::abs(self.origin[0] - self.plane[0]);
		let dy = f32::abs(self.origin[1] - self.plane[1]);
		if dx < 0.001 && dy > TILE_SIZE as f32 {
			return (DIAGONAL, vec!(-1.0, -1.0));
		}
		let slope =  dy / dx;
		let angle = f32::atan(slope);

		let mut base_side: f32;
		let mut delta_x_sign: f32 = 1.0;
		match self.h_dir {
			HorizontalDirection::Left => {
				base_side = self.plane[0] % TILE_SIZE as f32;
				delta_x_sign = -delta_x_sign;
			},
			HorizontalDirection::Right => {
				base_side = TILE_SIZE as f32 -
					self.plane[0] % TILE_SIZE as f32;
			}
		}
		let mut delta_y_sign: f32 = 1.0;
		match self.v_dir {
			VerticalDirection::Up => delta_y_sign = -delta_y_sign,
			VerticalDirection::Down => {}
		}
		let first_len = base_side / f32::cos(angle);
		let mut height_side = f32::sin(angle) * first_len;

		let mut collided_vec: Vec<f32> = vec!(self.plane[0] +
											  base_side *
											  delta_x_sign,
											  self.plane[1] +
											  height_side *
											  delta_y_sign);
		// if map.tile_from_vec(&collided_vec)
		if map.check_row_tile(&self, collided_vec.clone())
		{
			return (first_len, collided_vec);
		}

		let mut len_inc: f32 = first_len;
		let mut step_len = TILE_SIZE as f32 / f32::cos(angle);
		let mut step_height = f32::sin(angle) * step_len;

		let mut counter = 0;
		loop {
			len_inc = len_inc + step_len;
			counter = counter + 1;
			let vx = (base_side + (counter * TILE_SIZE) as f32) * delta_x_sign;
			let vy=(height_side + counter as f32 * step_height) * delta_y_sign;
			collided_vec = vec!(self.plane[0] + vx,
								self.origin[1] + vy);
			// if map.tile_from_vec(&collided_vec)
			if map.check_row_tile(&self, collided_vec.clone())
			{
				return (len_inc, collided_vec);
			}
		}
	}
	fn check_row_collision(&self, map: &Map) -> (f32, Vec<f32>) {
		let dx = f32::abs(self.origin[0] - self.plane[0]);
		let dy = f32::abs(self.origin[1] - self.plane[1]);
		if dy < 0.001 && dx > TILE_SIZE as f32 {
			return (DIAGONAL, vec!(-1.0, -1.0));
		}
		let slope =  dy / dx;
		let angle = f32::atan(slope);

		let mut height_side: f32;
		let mut delta_y_sign: f32 = 1.0;
		match self.v_dir {
			VerticalDirection::Down => {
				height_side = TILE_SIZE as f32 -
					self.plane[1] % TILE_SIZE as f32;
			},
			VerticalDirection::Up => {
				height_side = self.plane[1] % TILE_SIZE as f32;
				delta_y_sign = -delta_y_sign;
			}
		}
		let mut delta_x_sign: f32 = 1.0;
		match self.h_dir {
			HorizontalDirection::Left => delta_x_sign = -delta_x_sign,
			HorizontalDirection::Right => {}
		}
		let first_len = height_side / f32::sin(angle);
		let mut base_side = f32::cos(angle) * first_len;

		let mut collided_vec: Vec<f32> = vec!(self.plane[0] +
											  base_side *
											  delta_x_sign,
											  self.plane[1] +
											  height_side *
											  delta_y_sign);
		if map.check_row_tile(&self, collided_vec.clone())
		{
			return (first_len, collided_vec);
		}

		let mut len_inc: f32 = first_len;
		let mut step_len = TILE_SIZE as f32 / f32::sin(angle);
		let mut step_base = f32::cos(angle) * step_len;

		let mut counter = 0;
		loop {
			len_inc = len_inc + step_len;
			counter = counter + 1;
			let vx = (base_side + (counter as f32) * step_base) * delta_x_sign;
			let vy=(height_side + (counter * TILE_SIZE) as f32) * delta_y_sign;
			collided_vec = vec!(self.plane[0] + vx,
								self.plane[1] + vy);
			if map.check_row_tile(&self, collided_vec.clone())
			{
				return (len_inc, collided_vec);
			}
		}
	}
	// fn cast(&self) -> f32 {}
}

#[derive(Clone)]
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
	fn tile_from_vec(&self, v: &Vec<f32>) -> bool {
		let x: i32 = v[0].round() as i32 / TILE_SIZE;
		let y: i32 = v[1].round() as i32 / TILE_SIZE;
		let index: usize = (x + y * TILES_W) as usize;
		if x < 0 || x >= TILES_W || y < 0 || y >= TILES_H {
			return true;
		}
		self.tiles[index]
	}
	fn mapspace_to_tilespace(&self, point: Vec<f32>) -> (i32, i32) {
		let x: i32 = point[0].floor() as i32 / TILE_SIZE;
		let y: i32 = point[1].floor() as i32 / TILE_SIZE;
		(x, y)
	}
	fn check_column_tile(&self, ray: &Ray, point: Vec<f32>) -> bool {
		let (x , y) = self.mapspace_to_tilespace(point);
		let index: usize;
		match ray.h_dir {
			HorizontalDirection::Right => {
				index = (x + y * TILES_W) as usize;
			},
			HorizontalDirection::Left => {
				index = (x - 1 + y * TILES_W) as usize;
			}
		}
		if (index as i32) >= 0 && (index as i32) < TILES_W * TILES_H {
			self.tiles[index]
		} else {
			true
		}
	}
	fn check_row_tile(&self, ray: &Ray, point: Vec<f32>) -> bool {
		let (x , y) = self.mapspace_to_tilespace(point);
		let index: usize;
		match ray.v_dir {
			VerticalDirection::Down => {
				index = (x + y * TILES_W) as usize;
			},
			VerticalDirection::Up => {
				index = (x + (y - 1) * TILES_W) as usize;
			}
		}
		if (index as i32) >= 0 && (index as i32) < TILES_W * TILES_H {
			self.tiles[index]
		} else {
			true
		}
	}
	fn render(&self, ctx: &mut BTerm) {
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
	fn new(map: &Map) -> Self {
		Self {
			mode: GameMode::TwoD,
			player: Player::new(&map),
			map: map.clone(),
		}
	}
	fn three_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print(1, 1, "Hellow 3D World!");
	}
	fn two_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		self.map.render(ctx);
		self.player.update(ctx, &self.map);
		self.player.render(ctx);
		ctx.print(0, 0, "Hellow 2D World!");
		ctx.print(0, 1, format!("P: {}, {}",
								self.player.camera[0],
								self.player.camera[1]));
		ctx.print(0, 2, format!("D: {}, {}",
			self.player.camera[0] + self.player.direction[0],
			self.player.camera[1] + self.player.direction[1]));
		ctx.print(0, 3, format!("N: {}, {}",
			self.player.camera[0] + self.player.direction[0] - self.player.plane[0] / 2.0,
			self.player.camera[1] + self.player.direction[1] - self.player.plane[1] / 2.0));
	}
}

struct Player {
	camera: [f32; 2],
	direction: [f32; 2],
	plane: [f32; 2],
	begin: Vec<f32>,
	end: Vec<f32>,
}

impl Player {
	fn new(map: &Map) -> Self {
		let c = [10.0, 10.0];
		let d = [DIR_LENGTH, 0.0];
		let p = [0.0, f32::tan(0.436332) * DIR_LENGTH];
		let b = vec!(c[0] + d[0] + p[0] / 2.0,
					 c[1] + d[1] + p[1] / 2.0);
		Self {
			camera: c,
			direction: d,
			plane: p,
			begin: b,
			end: Ray::new(c, [c[0] + d[0] + p[0] / 2.0,
							  c[1] + d[1] + p[1] / 2.0])
				.check_column_collision(map).1,
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
	fn update(&mut self, ctx: &mut BTerm, map: &Map) {
		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::Q => {
					let point_at_plane = [
						self.camera[0]+self.direction[0]+self.plane[0]/2.0,
						self.camera[1]+self.direction[1]+self.plane[1]/2.0,
					];
					let ray_result: (f32, Vec<f32>);
					ray_result = Ray::new([self.camera[0], self.camera[1]],
										  point_at_plane)
						.check_column_collision(&map);
					ctx.set(ray_result.1[0].round() as i32,
							ray_result.1[1].round() as i32,
							RED, RED, 'X');
					let _ = std::process::Command::new("pause").status();
				}
				VirtualKeyCode::Z => {
					self.begin = vec!(self.camera[0] +
									  self.direction[0] +
									  self.plane[0] / 2.0,
									  self.camera[1] +
									  self.direction[1] +
									  self.plane[1] / 2.0);
					self.end = Ray::new(self.camera,[
						self.camera[0]+self.direction[0]+self.plane[0]/2.0,
						self.camera[1]+self.direction[1]+self.plane[1]/2.0])
					//.check_column_collision(map).1;
						.check_row_collision(map).1;
			}
				VirtualKeyCode::E => {
					map.tile_from_vec(&vec!(self.camera[0], self.camera[1]));
				}
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
			(self.direction[0] * 100.0).round() as i32,
			(self.direction[1] * 100.0).round() as i32
		);
		let pln = Point::new(
			(self.plane[0] / 2.0 * 100.0).round() as i32,
			(self.plane[1] / 2.0 * 100.0).round() as i32
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
		for point in Bresenham::new(Point::new(self.begin[0] as i32,
											   self.begin[1] as i32),
									Point::new(self.end[0] as i32,
											   self.end[1] as i32))
		{
			ctx.set(point.x, point.y, RED, BLACK, '*');
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
	main_loop(context, State::new(&Map::new()))
}
