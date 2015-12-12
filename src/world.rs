use id_map::IdMap;

use allegro::*;
use allegro_primitives::*;
use std::collections::HashSet;

pub static DT: f32 = 1.0 / 60.0;

pub struct Object
{
	pub remove_me: bool,
	
	pub has_pos: bool,
	pub x: f32,
	pub y: f32,
	
	pub has_vel: bool,
	pub vx: f32,
	pub vy: f32,
	
	pub debug_draw: bool,

	pub is_game: bool,
	pub started: bool,
	pub player_id: usize,
	
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
}

impl Object
{
	pub fn new() -> Object
	{
		Object
		{
			remove_me: false,
			
			has_pos: false,
			x: 0.0,
			y: 0.0,
			
			has_vel: false,
			vx: 0.0,
			vy: 0.0,
			
			debug_draw: false,

			is_game: false,
			started: false,
			player_id: 0,
			
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
	
	new_objects: Vec<(usize, Object)>,
	// This follows the object's ids.
	next_id: usize,
	ids_to_remove: HashSet<usize>,
	
	pub key_down: Option<KeyCode>,
	pub key_up: Option<KeyCode>,
	pub quit: bool,
	pub paused: bool,
	pub time: f32,
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
	pub fn new(core: Core, prim: PrimitivesAddon) -> World
	{
		World
		{
			state: WorldState
			{
				core: core,
				prim: prim,
				key_down: None,
				key_up: None,
				quit: false,
				paused: false,
				time: 0.0,
				new_objects: vec![],
				ids_to_remove: HashSet::new(),
				next_id: 1,
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
