use player::new_player;
use branch::new_branch;
use world::{DT, WIDTH};

use allegro::*;
use allegro_font::*;
use std::cmp::max;

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
			state.add_object(new_branch(0.0, 0.0, 64.0, -64.0, time));
			obj.started = true;
			obj.start_time = time;
		}
					
		let mut trans = Transform::identity();
		let scale = state.disp.get_width() as f32 / WIDTH;
		trans.scale(scale, scale);
		trans.translate(state.disp.get_width() as f32 / 2.0, state.disp.get_height() as f32);
		state.core.use_transform(&trans);
	}
}

simple_behavior!
{
	GameDraw[obj.is_game] |_id, obj, state|
	{
		let time_left = 60 - (state.time - obj.start_time) as i32;
		let time_left = max(0, time_left);
		let text = format!("{}", time_left);
		state.core.draw_text(&state.ui_font, state.core.map_rgb(255, 255, 255), 0.0, -200.0, FontAlign::Centre, &text);
	}
}
