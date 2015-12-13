use world::Object;

use world::{WorldState, random_color};

use allegro::*;

pub fn new_player(parent: usize, state: &WorldState) -> Object
{
	Object
	{
		has_pos: true,
		has_vel: true,
		is_player: true,
		can_want_move: true,
		affected_by_gravity: true,
		is_solid: true,
		//~ debug_draw: true,
		x: 0.0,
		y: -50.0,
		size: 15.0,
		parent: parent,
		sprite: Some(state.player.clone()),
		color: random_color(&state.core),
		..Object::new()
	}
}

simple_behavior!
{
	PlayerInput[obj.can_want_move && obj.is_player] |_id, obj, state|
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
