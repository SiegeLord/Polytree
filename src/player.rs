use world::Object;

use allegro::*;

pub fn new_player() -> Object
{
	Object
	{
		has_pos: true,
		has_vel: true,
		is_player: true,
		can_want_move: true,
		affected_by_gravity: true,
		is_solid: true,
		debug_draw: true,
		x: 400.0,
		y: 400.0,
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
