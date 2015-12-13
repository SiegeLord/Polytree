use id_map::IdMap;

use allegro::*;
use allegro_sys::*;
use allegro_primitives::*;
use allegro_font::*;
use allegro_ttf::*;
use std::collections::HashSet;
use std::rc::Rc;

use rand::{self, Rng};

pub const DT: f32 = 1.0 / 60.0;
pub const WIDTH: f32 = 2000.0;
pub const MAX_SPEED: f32 = 256.0;
pub const DEATH: f32 = 700.0;
pub const DURATION: f32 = 30.0;
pub const BOSS_RX: f32 = WIDTH / 3.0;
pub const BOSS_RY: f32 = DEATH / 4.0;
pub const BOSS_RATE: f32 = 10.0;

pub struct Object
{
	pub parent: usize,
	
	pub has_pos: bool,
	pub x: f32,
	pub y: f32,
	
	pub has_vel: bool,
	pub vx: f32,
	pub vy: f32,
	pub ax: f32,
	pub ay: f32,
	
	pub debug_draw: bool,

	pub is_game: bool,
	pub player_id: usize,
	pub start_time: f32,
	pub stage: i32,
	
	pub can_want_move: bool,
	pub want_move_left: bool,
	pub want_move_right: bool,

	pub is_player: bool,

	pub is_branch: bool,
	pub branch_start_x: f32,
	pub branch_start_y: f32,
	pub branch_dir_x: f32,
	pub branch_dir_y: f32,
	pub branch_start_time: f32,
	pub branch_max_dur: f32,
	pub branch_spawns: i32,
	
	pub affected_by_gravity: bool,
	pub is_solid: bool,
	pub size: f32,
	
	pub is_dollar: bool,
	pub dollar_spawn_color: Color,
	
	pub is_boss: bool,
	
	pub sprite: Option<Rc<Bitmap>>,
	pub color: Color,
}

impl Object
{
	pub fn new() -> Object
	{
		Object
		{
			parent: 0,
			
			has_pos: false,
			x: 0.0,
			y: 0.0,
			
			has_vel: false,
			vx: 0.0,
			vy: 0.0,
			ax: 0.0,
			ay: 0.0,
			
			debug_draw: false,

			is_game: false,
			player_id: 0,
			start_time: 0.0,
			stage: 0,
			
			can_want_move: false,
			want_move_left: false,
			want_move_right: false,
			
			is_player: false,
			
			is_branch: false,
			branch_start_x: 0.0,
			branch_start_y: 0.0,
			branch_dir_x: 0.0,
			branch_dir_y: 0.0,
			branch_start_time: 0.0,
			branch_max_dur: 0.0,
			branch_spawns: 0,
			
			affected_by_gravity: false,
			is_solid: false,
			size: 10.0,

			is_dollar: false,
			dollar_spawn_color: Color(ALLEGRO_COLOR{r: 0.0, g: 0.0, b: 0.0, a: 0.0}),
			
			is_boss: false,
			
			sprite: None,
			color: Color(ALLEGRO_COLOR{r: 0.0, g: 0.0, b: 0.0, a: 0.0}),
		}
	}
}

macro_rules! simple_behavior
{
	($name: ident[$check: expr] |$id: ident, $obj: ident, $state: ident| $e: expr) =>
	{
		pub struct $name;
		
		impl ::world::Behavior<::world::Object, ::world::WorldState> for $name
		{
			fn check_object(&self, $obj: &::world::Object) -> bool
			{
				$check
			}
			
			fn handle_objects(&mut self, objects: &mut ::id_map::IdMap<::world::Object>, $state: &mut ::world::WorldState)
			{
				for &mut ($id, ref mut $obj) in objects.elems_mut()
				{
					if self.check_object($obj)
					{
						$e
					}
				}
			}
		}
	}
}

pub trait Behavior<O, S>
{
	fn check_object(&self, &O) -> bool
	{
		true
	}
	
	fn handle_objects(&mut self, objects: &mut IdMap<O>, state: &mut S);
}

pub struct WorldState
{
	pub core: Core,
	pub prim: PrimitivesAddon,
	pub disp: Display,
	pub ttf: TtfAddon,
	
	new_objects: Vec<(usize, Object)>,
	// This follows the object's ids.
	next_id: usize,
	ids_to_remove: HashSet<usize>,
	
	pub key_down: Option<KeyCode>,
	pub key_up: Option<KeyCode>,
	pub quit: bool,
	pub paused: bool,
	pub time: f32,
	pub ui_font: Font,
	pub dollar: Rc<Bitmap>,
	pub boss: Rc<Bitmap>,
	pub player: Rc<Bitmap>,
}

impl WorldState
{
	pub fn add_object(&mut self, obj: Object) -> usize
	{
		let id = self.next_id;
		self.new_objects.push((id, obj));
		self.next_id += 1;
		id
	}
	
	pub fn remove_object(&mut self, id: usize)
	{
		self.ids_to_remove.insert(id);
	}
}

pub struct World
{
	objects: IdMap<Object>,
	logic_behaviors: Vec<Box<Behavior<Object, WorldState>>>,
	input_behaviors: Vec<Box<Behavior<Object, WorldState>>>,
	draw_behaviors: Vec<Box<Behavior<Object, WorldState>>>,
	pub state: WorldState,
}

impl World
{
	pub fn new(core: Core, prim: PrimitivesAddon, disp: Display, ttf: TtfAddon) -> World
	{
		let font_path = "data/Energon.ttf";
		let dollar_path = "data/dollar.png";
		let boss_path = "data/boss.png";
		let player_path = "data/player.png";
		let ui_font = ttf.load_ttf_font(font_path, 128, TtfFlags::zero()).expect(&format!("Couldn't load {}", font_path));
		let dollar = Bitmap::load(&core, dollar_path).expect(&format!("Couldn't load {}", dollar_path));
		let boss = Bitmap::load(&core, boss_path).expect(&format!("Couldn't load {}", boss_path));
		let player = Bitmap::load(&core, player_path).expect(&format!("Couldn't load {}", player_path));
		World
		{
			state: WorldState
			{
				core: core,
				prim: prim,
				disp: disp,
				ttf: ttf,
				ui_font: ui_font,
				key_down: None,
				key_up: None,
				quit: false,
				paused: false,
				time: 0.0,
				new_objects: vec![],
				ids_to_remove: HashSet::new(),
				next_id: 1,
				dollar: Rc::new(dollar),
				boss: Rc::new(boss),
				player: Rc::new(player),
			},
			objects: IdMap::new(),
			logic_behaviors: vec![],
			input_behaviors: vec![],
			draw_behaviors: vec![],
		}
	}
	
	pub fn add_logic_behavior(&mut self, behavior: Box<Behavior<Object, WorldState>>)
	{
		self.logic_behaviors.push(behavior);
	}
	
	pub fn logic(&mut self)
	{
		for behavior in &mut self.logic_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
		
		for (expected_id, obj) in self.state.new_objects.drain(..)
		{
			let actual_id = self.objects.insert(obj);	
			assert_eq!(actual_id, expected_id);
		}
		self.state.next_id = self.objects.next_id();
		
		let mut old_ids_to_remove = vec![];
		old_ids_to_remove.extend(&self.state.ids_to_remove);
		let mut new_ids_to_remove = vec![];
		while !old_ids_to_remove.is_empty()
		{
			new_ids_to_remove.clear();
			for &(id, ref obj) in self.objects.elems()
			{
				for &dead_id in &old_ids_to_remove
				{
					if obj.parent == dead_id
					{
						new_ids_to_remove.push(id);
					}
				}
			}
			old_ids_to_remove.clear();
			old_ids_to_remove.extend(&new_ids_to_remove);
			self.state.ids_to_remove.extend(&new_ids_to_remove);
		}
		
		for id in self.state.ids_to_remove.drain()
		{
			if self.objects.get(id).is_some()
			{
				self.objects.remove(id);
			}
		}
	}
	
	pub fn add_input_behavior(&mut self, behavior: Box<Behavior<Object, WorldState>>)
	{
		self.input_behaviors.push(behavior);
	}
	
	pub fn input(&mut self)
	{
		for behavior in &mut self.input_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
	}
	
	pub fn add_draw_behavior(&mut self, behavior: Box<Behavior<Object, WorldState>>)
	{
		self.draw_behaviors.push(behavior);
	}
	
	pub fn draw(&mut self)
	{
		for behavior in &mut self.draw_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
	}
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32)
{
	let mut hue = hue % 360.0;
	if hue < 0.0
	{
		hue += 360.0;
	}
	let d = hue / 60.0;
	let e = hue / 60.0 - d;
	let a = value * (1.0 - saturation);
	let b = value * (1.0 - e * saturation);
	let c = value * (1.0 - (1.0 - e) * saturation);
	match d as i32
	{
		0 => (value, c, a),
		1 => (b, value, a),
		2 => (a, value, c),
		3 => (a, b, value),
		4 => (c, a, value),
		5 => (value, a, b),
		_ => unreachable!()
	}
}

pub fn random_color(core: &Core) -> Color
{
	let mut rng = rand::thread_rng();
	let (r, g, b) = hsv_to_rgb(rng.gen_range(0.0, 360.0), 0.5, 1.0);
	core.map_rgb_f(r, g, b)
}
