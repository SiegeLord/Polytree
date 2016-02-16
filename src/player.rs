// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use world::Object;

use world::{WorldState, random_color};

use allegro::*;

pub fn new_player(parent: usize, state: &mut WorldState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.has_pos = true;
	obj.has_vel = true;
	obj.is_player = true;
	obj.can_want_move = true;
	obj.affected_by_gravity = true;
	obj.is_solid = true;
	//~ debug_draw = true;
	obj.x = 0.0;
	obj.y = -50.0;
	obj.old_x = 0.0;
	obj.old_y = -50.0;
	obj.size = 15.0;
	obj.parent = parent;
	obj.sprite = Some(state.bitmap_manager.load(&state.core, "data/player.png").unwrap());
	obj.color = random_color();
	obj
}

simple_behavior!
{
	PlayerInput[obj.can_want_move && obj.is_player] |obj, state|
	{
		if let Some(KeyCode::Left) = state.key_down
		{
			obj.want_move_left = true;
		}
		if let Some(KeyCode::Right) = state.key_down
		{
			obj.want_move_right = true;
		}
		if let Some(KeyCode::Left) = state.key_up
		{
			obj.want_move_left = false;
		}
		if let Some(KeyCode::Right) = state.key_up
		{
			obj.want_move_right = false;
		}
	}
}
