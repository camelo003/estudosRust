use bracket_lib::prelude::*;

enum GameMode {
	Menu,
	Playing,
	End,
}

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 1000.0 / 24.0;

struct Player {
	x: i32,
	y: i32,
	velocity: f32,
}

impl Player {
	fn new(x: i32, y: i32) -> Self {
		Self {
			x,
			y,
			velocity: 0.0,
		}
	}
	fn render(&mut self, ctx: &mut BTerm) {
		if self.velocity < -1.0 {
			ctx.set(self.x - 1, self.y - 1, YELLOW, NAVY, to_cp437('_'));
			ctx.set(self.x - 2, self.y - 1, YELLOW, NAVY, to_cp437('_'));
		}else{
			ctx.set(self.x - 1, self.y - 2, YELLOW, NAVY, to_cp437('\\'));
			ctx.set(self.x, self.y - 1, YELLOW, NAVY, to_cp437('\\'));
		}
		ctx.set(self.x + 1, self.y - 1, YELLOW, NAVY, to_cp437('0'));
		ctx.set(self.x + 2, self.y - 1, YELLOW, NAVY, to_cp437('>'));
		ctx.set(self.x - 1, self.y, YELLOW, NAVY, to_cp437('/'));
		ctx.set(self.x, self.y, YELLOW, NAVY, to_cp437(')'));
		ctx.set(self.x - 1, self.y + 1, YELLOW, NAVY, to_cp437('"'));
		ctx.set(self.x - 2, self.y + 1, YELLOW, NAVY, to_cp437('"'));
	}
	fn gravity_and_move(&mut self) {
		if self.velocity < 2.0 {
			self.velocity += 0.2;
		}
		self.y += self.velocity as i32;
		self.x += 1;
		self.x = self.x % SCREEN_WIDTH;
		if self.y < 0 {
			self.y = 0;
		}
	}
	fn flap(&mut self) {
		self.velocity = -2.0;
	}
}

struct Obstacle {
	x: i32,
	gap_y: i32,
	size: i32,
}

impl Obstacle {
	fn new (x: i32, score: i32) -> Self {
		let mut random = RandomNumberGenerator::new();
		Self {
			x,
			gap_y: random.range(10,40),
			size: i32::max(2, 20 - score),
		}
	}
	fn render (&self, ctx: &mut BTerm) {
		for y in 0..self.gap_y - self.size / 2 {
			ctx.print_color(self.x - 1, y, RED, NAVY, "###");
		}
		for y in (self.gap_y + self.size / 2)..SCREEN_HEIGHT {
			ctx.print_color(self.x - 1, y, RED, NAVY, "###");
		}
	}
	fn hit_obstacle (&self, player: &Player) -> bool {
		self.x == player.x && i32::abs(self.gap_y - player.y) > self.size/ 2
	}
}

struct State {
	player: Player,
	frame_time: f32,
	mode: GameMode,
	score: i32,
	obstacles: Vec<Obstacle>,
}

impl State {
	fn new() -> Self {
		Self {
			player: Player::new(5, 25),
			frame_time: 0.0,
			mode: GameMode::Menu,
			score: 0,
			obstacles: vec![Obstacle::new(SCREEN_WIDTH / 2, 0)],
		}
	}
	fn play(&mut self, ctx: &mut BTerm) {
		ctx.cls_bg(NAVY);
		self.frame_time += ctx.frame_time_ms;
		if self.frame_time > FRAME_DURATION {
			self.frame_time = 0.0;
			self.player.gravity_and_move();
			if self.player.x == 0 {
				self.score+=1;
				self.place_obstacles();
			}
		}
		if let Some(VirtualKeyCode::Space) = ctx.key {
			self.player.flap();
		}
		for i in 0..self.obstacles.len() {
			self.obstacles[i].render(ctx);
		}
		self.player.render(ctx);
		ctx.print(0, 0,"Press SPACE: to fly.");
		ctx.print(0, 1,format!("Score: {}", self.score));
		for i in 0..self.obstacles.len() {
			if self.obstacles[i].x < self.player.x {
				continue;
			}
			if self.obstacles[i].hit_obstacle(&self.player) {
				self.mode = GameMode::End;
			}
		}
		if self.player.y > SCREEN_HEIGHT {
			self.mode = GameMode::End;
		}
	}
	fn restart(&mut self) {
		self.player = Player::new(5, 25);
		self.frame_time = 0.0;
		for _i in 0..self.obstacles.len() {
			self.obstacles.pop();
		}
		self.obstacles.push(Obstacle::new(SCREEN_WIDTH / 2, 0));
		self.score = 0;
		self.mode = GameMode::Playing;
	}
	fn main_menu(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print_centered(5, "Welcometo Flappy Dragon!");
		ctx.print_centered(8, "(P) Play Game");
		ctx.print_centered(9, "(Q) Quit Game");
		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Q => ctx.quitting = true,
				_ => {},
			}
		}
	}
	fn dead(&mut self, ctx: &mut BTerm) {
		ctx.cls();
		ctx.print_centered(5, "You are dead!");
		ctx.print_centered(8, format!("Your score: {}", self.score));
		ctx.print_centered(11, "(P) Play Game");
		ctx.print_centered(12, "(Q) Quit Game");
		if let Some(key) = ctx.key {
			match key {
				VirtualKeyCode::P => self.restart(),
				VirtualKeyCode::Q => ctx.quitting = true,
				_ => {},
			}
		}
	}
	fn place_obstacles(&mut self) {
		const CHANCES: [i32; 12] = [1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4];
		let mut random_gen = RandomNumberGenerator::new();
		let random_num: i32;
		let obstacles_num: i32;
		let interval: i32;

		for _i in 0..self.obstacles.len() {
			self.obstacles.pop();
		}
		random_num = random_gen.range(0, 12);
		obstacles_num = CHANCES[random_num as usize];
		interval = SCREEN_WIDTH / (obstacles_num + 1);
		for i in 1..=obstacles_num {
			self.obstacles.push(Obstacle::new(interval * i, self.score));
		}
	}
}

impl GameState for State {
	fn tick (&mut self, ctx: &mut BTerm) {
		match self.mode {
			GameMode::Menu => self.main_menu(ctx),
			GameMode::End => self.dead(ctx),
			GameMode::Playing => self.play(ctx),
		}
	}
}

fn main() -> BError {
	let context = BTermBuilder::simple80x50()
		.with_title("Flappy Dragon").
		build()?;
	main_loop(context, State::new())
}
