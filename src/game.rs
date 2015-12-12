use player::new_player;

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
			obj.started = true;
		}
	}
}
