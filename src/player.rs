use world::Object;

use allegro::*;

pub fn new_player() -> Object
{
	let mut player = Object::new();
	player.has_pos = true;
	player.has_vel = true;
	player.is_player = true;
	player.can_want_move = true;
	player.debug_draw = true;
	player.x = 100.0;
	player.y = 100.0;
	player
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
