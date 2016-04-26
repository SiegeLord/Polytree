// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use engine::world::WorldState;
use engine::bitmap_cache::BitmapCache;
use engine::id_map::{HasId, IdMint, UniqueId};

use allegro::*;
use allegro_primitives::*;
use allegro_font::*;
use allegro_ttf::*;
use rand::{self, Rng};
use std::collections::HashSet;
use std::rc::Rc;

pub const DT: f32 = 1.0 / 60.0;
pub const WIDTH: f32 = 2000.0;
pub const MAX_SPEED: f32 = 512.0;
pub const DEATH: f32 = 700.0;
pub const DURATION: f32 = 60.0;
pub const BOSS_RX: f32 = WIDTH / 3.0;
pub const BOSS_RY: f32 = DEATH / 4.0;
pub const BOSS_RATE: f32 = 10.0;

macro_rules! simple_behavior
{
	($name: ident[$check: expr] |$obj: ident, $state: ident| $e: expr) =>
	{
		pub struct $name;

		impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for $name
		{
			fn check_object(&self, $obj: &::game_state::Object) -> bool
			{
				$check
			}

			fn handle_objects(&mut self, objects: &mut ::engine::id_map::IdMap<::game_state::Object>, $state: &mut ::game_state::GameState)
			{
				for $obj in objects.elems_mut()
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

macro_rules! complex_behavior
{
	($name: ident[$check: expr] |$self_: ident, $obj: ident, $objects: ident, $state: ident| $e: expr) =>
	{
		impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for $name
		{
			fn check_object(&$self_, $obj: &::game_state::Object) -> bool
			{
				$check
			}

			fn handle_objects(&mut $self_, $objects: &mut ::engine::id_map::IdMap<::game_state::Object>, $state: &mut ::game_state::GameState)
			{
				$e
			}
		}
	}
}

macro_rules! object
{
	($name: ident
	{
		$($member: ident : $type_ : ty = $init: expr),* $(,)*
	}) =>
	{
		pub struct $name
		{
			id: UniqueId,
			$(pub $member : $type_),*
		}

		impl $name
		{
			pub fn new(id: UniqueId) -> $name
			{
				$name
				{
					id: id,
					$($member : $init),*
				}
			}
		}
	}
}

object!
{
	Object
	{
		parent: usize = 0,

		has_pos: bool = false,
		x: f32 = 0.0,
		y: f32 = 0.0,
		old_x: f32 = 0.0,
		old_y: f32 = 0.0,

		has_vel: bool = false,
		vx: f32 = 0.0,
		vy: f32 = 0.0,
		ax: f32 = 0.0,
		ay: f32 = 0.0,

		debug_draw: bool = false,

		is_game: bool = false,
		player_id: usize = 0,
		start_time: f32 = 0.0,
		stage: i32 = 0,

		can_want_move: bool = false,
		want_move_left: bool = false,
		want_move_right: bool = false,

		is_player: bool = false,

		is_branch: bool = false,
		branch_start_x: f32 = 0.0,
		branch_start_y: f32 = 0.0,
		branch_dir_x: f32 = 0.0,
		branch_dir_y: f32 = 0.0,
		branch_start_time: f32 = 0.0,
		branch_max_dur: f32 = 0.0,
		branch_spawns: i32 = 0,

		affected_by_gravity: bool = false,
		is_solid: bool = false,
		size: f32 = 10.0,

		is_dollar: bool = false,
		dollar_spawn_color: Color = Color::from_rgba(0, 0, 0, 0),

		is_boss: bool = false,

		sprite: Option<Rc<Bitmap>> = None,
		color: Color = Color::from_rgba(0, 0, 0, 0),
	}
}

impl HasId for Object
{
	fn get_id(&self) -> usize
	{
		self.id.get()
	}
}

pub struct GameState
{
	pub core: Core,
	pub prim: PrimitivesAddon,
	pub disp: Display,
	pub ttf: TtfAddon,

	pub id_mint: IdMint,

	new_objects: Vec<Object>,
	ids_to_remove: HashSet<usize>,

	pub key_down: Option<KeyCode>,
	pub key_up: Option<KeyCode>,
	pub quit: bool,
	pub paused: bool,
	pub time: f32,
	pub draw_interp: f32,
	pub ui_font: Font,
	pub bitmap_manager: BitmapCache,
}

impl GameState
{
	pub fn new(core: Core, prim: PrimitivesAddon, disp: Display, ttf: TtfAddon) -> GameState
	{
		let font_path = "data/Energon.ttf";
		let ui_font = ttf.load_ttf_font(font_path, 64, TtfFlags::zero()).expect(&format!("Couldn't load {}", font_path));
		GameState
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
			draw_interp: 0.0,
			new_objects: vec![],
			ids_to_remove: HashSet::new(),
			id_mint: IdMint::new(),
			bitmap_manager: BitmapCache::new(),
		}
	}

	pub fn add_object(&mut self, obj: Object)
	{
		self.new_objects.push(obj);
	}

	pub fn remove_object(&mut self, id: usize)
	{
		self.ids_to_remove.insert(id);
	}

	pub fn new_id(&mut self) -> UniqueId
	{
		self.id_mint.new_id()
	}
}

impl WorldState<Object> for GameState
{
	fn get_new_objects(&mut self) -> &mut Vec<Object>
	{
		&mut self.new_objects
	}

	fn get_ids_to_remove(&mut self) -> &mut HashSet<usize>
	{
		&mut self.ids_to_remove
	}
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32)
{
	let mut hue = hue % 360.0;
	if hue < 0.0
	{
		hue += 360.0;
	}
	let d = (hue / 60.0).floor();
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

pub fn random_color() -> Color
{
	let mut rng = rand::thread_rng();
	let (r, g, b) = hsv_to_rgb(rng.gen_range(0.0, 360.0), 0.5, 1.0);
	info!("Chosen: {} {} {}", r, g, b);
	Color::from_rgb_f(r, g, b)
}
