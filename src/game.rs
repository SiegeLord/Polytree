use player::new_player;
use branch::new_branch;

use allegro::*;

simple_behavior!
{
	GameInput[obj.is_game] |_id, obj, state|
	{
		if let Some(KeyCode::Escape) = state.key_down
		{
			state.quit = true;
		}
		
		if let Some(KeyCode::Space) = state.key_down
		{
			state.remove_object(obj.player_id);
		}
	}
}

simple_behavior!
{
	GameLogic[obj.is_game] |_id, obj, state|
	{
		if !obj.started
		{
			obj.player_id = state.add_object(new_player());
			let time = state.time;
			state.add_object(new_branch(400.0, 800.0, 64.0, -64.0, time));
			state.add_object(new_branch(500.0, 700.0, -64.0, -64.0, time));
			obj.started = true;
		}
	}
}
