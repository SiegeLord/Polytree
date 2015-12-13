use player::new_player;
use branch::new_branch;
use world::{Object, WorldState, DEATH, WIDTH, DURATION};

use allegro::*;
use allegro_font::*;

pub fn start_stage(stage: i32, state: &mut WorldState)
{
	let player_id = state.add_object(new_player());
	let time = state.time;
	let mut branch = new_branch(0.0, 0.0, 128.0, -96.0, time);
	branch.branch_spawns = 2;
	state.add_object(branch);
	info!("Starting stage: {}", stage);
	let stage = Object
	{
		is_game: true,
		stage: stage,
		player_id: player_id,
		start_time: time,
		..Object::new()
	};
	state.add_object(stage);
}

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

pub struct GameLogic;

impl ::world::Behavior<::world::Object, ::world::WorldState> for GameLogic
{
	fn check_object(&self, obj: &::world::Object) -> bool
	{
		obj.is_game
	}
	
	fn handle_objects(&mut self, objects: &mut ::id_map::IdMap<::world::Object>, state: &mut ::world::WorldState)
	{
		let mut time_left = 0.0;
		let mut game_id = 0;
		let mut player_id = 0;
		let mut stage = 0;
		for &mut (id, ref mut obj) in objects.elems_mut()
		{
			if self.check_object(obj)
			{
				game_id = id;
				stage = obj.stage;
				
				time_left = DURATION - (state.time - obj.start_time);
				player_id = obj.player_id;
				
				let mut trans = Transform::identity();
				let scale = state.disp.get_width() as f32 / WIDTH;
				trans.scale(scale, scale);
				trans.translate(state.disp.get_width() as f32 / 2.0, state.disp.get_height() as f32);
				state.core.use_transform(&trans);
				break;
			}
		}
		
		if game_id == 0
		{
			return;
		}

		if time_left < 0.0 && time_left > -0.1
		{
			if let Some(player) = objects.get(player_id)
			{
				if player.y > -DEATH
				{
					state.remove_object(player_id);
				}
			}
		}
		
		if time_left < -2.0
		{
			info!("Here");
			state.remove_object(game_id);
			let advance = objects.get(player_id).map_or(0, |_| 1);
			start_stage(stage + advance, state);
		}
	}
}

simple_behavior!
{
	GameDraw[obj.is_game] |_id, obj, state|
	{
		let time_left = DURATION - (state.time - obj.start_time);
		let scale = state.disp.get_width() as f32 / WIDTH;
		
		if time_left.abs() < 1.0
		{
			let a = 1.0 - time_left.abs() / 1.0;
			let c = state.core.map_rgba_f(a, a, a, a);
			state.prim.draw_filled_rectangle(-WIDTH / 2.0, 0.0, WIDTH / 2.0, -DEATH, c);
		}
		
		if time_left < -1.0
		{
			let mut a = -(time_left + 1.0);
			if a > 1.0
			{
				a = 1.0;
			}
			let c = state.core.map_rgba_f(0.0, 0.0, 0.0, a);
			let y = state.disp.get_height() as f32 / scale;
			state.prim.draw_filled_rectangle(-WIDTH / 2.0, 0.0, WIDTH / 2.0, -y, c);
		}

		let time_left = if time_left < 0.0
		{
			0.0
		}
		else
		{
			time_left
		};
		let time_text = format!("{}", time_left.ceil() as i32);
		let stage_text = format!("STAGE {}", obj.stage);
		
		let y = state.disp.get_height() as f32 / scale;
		let x1 = -WIDTH / 2.0 + 50.0;
		let x2 = WIDTH / 2.0 - 50.0;
		
		state.core.draw_text(&state.ui_font, state.core.map_rgba(64, 64, 64, 64), x1, -y + 50.0, FontAlign::Left, &stage_text);
		state.core.draw_text(&state.ui_font, state.core.map_rgba(64, 64, 64, 64), x2, -y + 50.0, FontAlign::Right, &time_text);
	}
}
