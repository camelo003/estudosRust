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
const TILES_CUBE: i32 = TILES_W * TILES_H;

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
		if dx == 0.0 {
			return (DIAGONAL, vec!(-1.0, -1.0));
		}
		let slope: f32;
		let angle: f32;
		if dx == 0.0 {
			angle = 0.0;
		} else {
			slope =  dy / dx;
			angle = f32::atan(slope);
		}

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
		if map.check_column_tile(&self, collided_vec.clone())
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
			if map.check_column_tile(&self, collided_vec.clone())
			{
				return (len_inc, collided_vec);
			}
		}
	}
	fn check_row_collision(&self, map: &Map) -> (f32, Vec<f32>) {
		let dx = f32::abs(self.origin[0] - self.plane[0]);
		let dy = f32::abs(self.origin[1] - self.plane[1]);
		if dy == 0.0 {
			return (DIAGONAL, vec!(-1.0, -1.0));
		}
		let slope: f32;
		let angle: f32;
		if dx == 0.0 {
			angle = 0.0;
		} else {
			slope =  dy / dx;
			angle = f32::atan(slope);
		}

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
	fn cast(&self, map: &Map) -> (f32, Vec<f32>) {
		let (column_len, column_point) = self.check_column_collision(map);
		let (row_len, row_point) = self.check_row_collision(map);
		if column_len < row_len {
			(column_len, column_point)
		} else {
			(row_len, row_point)
		}
	}
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
				index = ((x - 1) + y * TILES_W) as usize;
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
	fn render_2d(&self, ctx: &mut BTerm) {
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
	fn map_to_range(s: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32{
		b1 + (s - a1) * (b2 - b1)/(a2 - a1)

	}
	fn render_3d(&self, ctx: &mut BTerm, ply: &Player) {
		for i in 0..SCREEN_W {
			let line_h = Self::map_to_range(ply.dist[i as usize],
											0.0,
											DIAGONAL * 0.75,
											SCREEN_H as f32,
											0.0);
			let line_offset = (SCREEN_H as f32 - line_h) / 2.0;
				let line: Bresenham = Bresenham::new(
					Point {x: i, y: line_offset as i32},
					Point {x: i, y: SCREEN_H - line_offset as i32});
			line.for_each(|p: Point| ctx.set(p.x, p.y, GREY, BLACK, '#'));
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
			mode: GameMode::ThreeD,
			player: Player::new(&map),
			map: map.clone(),
		}
	}
	fn three_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		self.player.update(ctx, &self.map);
		self.map.render_3d(ctx, &self.player);
		ctx.print(1, 1, "Hellow 3D World!");
	}
	fn two_d(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		self.map.render_2d(ctx);
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
	begin: Vec<Vec<f32>>,
	end: Vec<Vec<f32>>,
	dist: Vec<f32>
}

impl Player {
	fn lerp_f32(a: f32, b: f32, t:f32) -> f32 {
		a + t * (b - a)
	}
	fn cast_all(c: [f32; 2],
				d: [f32; 2],
				p: [f32; 2],
				map: &Map) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<f32>) {
		let mut b:  Vec<Vec<f32>> = Vec::new();
		let mut e:  Vec<Vec<f32>> = Vec::new();
		let mut dt: Vec<f32> = Vec::new();
		let w: f32 = 1.0 / SCREEN_W as f32;
		for i in 0..SCREEN_W {
			let lerp_x: f32;
			let lerp_y: f32;
			let t: f32 = w * i as f32; // FIXME? compensar ultimo ray?
			lerp_x = Self::lerp_f32(-p[0] / 2.0, p[0] / 2.0, t);
			lerp_y = Self::lerp_f32(-p[1] / 2.0, p[1] / 2.0, t);
			b.push(vec!(c[0] + d[0] + lerp_x,
						c[1] + d[1] + lerp_y));
			let ray = Ray::new(c, [c[0] + d[0] + lerp_x,
								   c[1] + d[1] + lerp_y]).cast(map);
			e.push(ray.1);
			dt.push(ray.0);
		}
		(b, e, dt)
	}
	fn new(map: &Map) -> Self {
		let c = [10.0, 10.0];
		let d = [DIR_LENGTH, 0.0];
		let p = [0.0, f32::tan(0.436332) * DIR_LENGTH];
		let (b, e, dt) = Self::cast_all(c, d, p, map);
		Self {
			camera: c,
			direction: d,
			plane: p,
			begin: b,
			end: e,
			dist: dt,
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
				VirtualKeyCode::Z => {
					for i in 0..self.dist.len() {
						println!("{}: {}", i, self.dist[i]);
					}
				},
				VirtualKeyCode::W => {
					self.camera = Self::sum(
						self.camera,
						Self::normalize(self.direction)
					);
					(self.begin, self.end, self.dist) =
						Self::cast_all(self.camera,
									   self.direction,
									   self.plane,
									   map);
				},
				VirtualKeyCode::S => {
					self.camera = Self::sub(
						self.camera,
						Self::normalize(self.direction)
					);
					(self.begin, self.end, self.dist) =
						Self::cast_all(self.camera,
									   self.direction,
									   self.plane,
									   map);
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
					(self.begin, self.end, self.dist) =
						Self::cast_all(self.camera,
									   self.direction,
									   self.plane,
									   map);
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
					(self.begin, self.end, self.dist) =
						Self::cast_all(self.camera,
									   self.direction,
									   self.plane,
									   map);
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
		for point in Bresenham::new(pos + dir + pln, pos + dir - pln) {
			ctx.set(point.x, point.y, WHITE, BLACK, '*');
		}
		for i in 0..self.begin.len() {
			for point in Bresenham::new(Point::new(self.begin[i][0] as i32,
												   self.begin[i][1] as i32),
										Point::new(self.end[i][0] as i32,
												   self.end[i][1] as i32))
			{
				ctx.set(point.x, point.y, RED, BLACK, '*');
			}
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
	main_loop(context, State::new(&Map::new()))
}
