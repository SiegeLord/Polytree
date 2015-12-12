use player::new_player;

use allegro::*;

simple_behavior!
{
	GameInput[obj.is_game] |_id, obj, state|
	{
		if let Some(KeyCode::Escape) = state.key_up
		{
			state.quit = true;
		}
	}
}

simple_behavior!
{
	GameLogic[obj.is_game] |_id, obj, state|
	{
		if !obj.started
		{
			state.add_object(new_player());
			obj.started = true;
		}
	}
}
