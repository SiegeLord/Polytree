// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use player::new_player;
use branch::new_branch;
use dollar::new_dollar;
use boss::new_boss;
use world::{Object, WorldState, DT, DEATH, WIDTH, DURATION, random_color};
use id_map::HasId;

use allegro::*;
use allegro_font::*;
use rand::{self, Rng};

pub fn start_stage(stage: i32, state: &mut WorldState)
{
	let time = state.time;
	let stage_uniq_id = state.new_id();
	let player = new_player(stage_uniq_id.get(), state);
	let player_id = player.get_id();
	state.add_object(player);
	info!("Starting stage: {}", stage);
	let dollar_color = random_color();

	let mut stage_obj = Object::new(stage_uniq_id);
	stage_obj.is_game = true;
	stage_obj.stage = stage;
	stage_obj.player_id = player_id;
	stage_obj.start_time = time;
	stage_obj.dollar_spawn_color = dollar_color;

	let stage_id = stage_obj.get_id();
	state.add_object(stage_obj);
	let mut branch = new_branch(stage_id, random_color(), -200.0, 0.0, -256.0, -192.0, time, state.new_id());
	branch.branch_spawns = 2;
	state.add_object(branch);
	let mut branch = new_branch(stage_id, random_color(), 200.0, 0.0, 256.0, -192.0, time, state.new_id());
	branch.branch_spawns = 2;
	state.add_object(branch);
	if stage % 3 == 0
	{
		let boss = new_boss(stage_id, dollar_color, state);
		state.add_object(boss);
	}
}

simple_behavior!
{
	GameInput[obj.is_game] |obj, state|
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
		// Can't access other objects in a mutable way... this happens every time, there must be a better way.
		let mut time_left = 0.0;
		let mut game_id = 0;
		let mut player_id = 0;
		let mut stage = 0;
		for obj in objects.elems_mut()
		{
			if self.check_object(obj)
			{
				game_id = obj.get_id();
				stage = obj.stage;
				
				time_left = DURATION - (state.time - obj.start_time);
				player_id = obj.player_id;
				
				let mut trans = Transform::identity();
				let scale = state.disp.get_width() as f32 / WIDTH;
				trans.scale(scale, scale);
				trans.translate(state.disp.get_width() as f32 / 2.0, state.disp.get_height() as f32);
				state.core.use_transform(&trans);
				
				let mut rng = rand::thread_rng();
				if rng.gen::<f32>() < 1.5 * DT
				{
					for _ in 0..obj.stage
					{
						let x = *rng.choose(&[-WIDTH - 10.0, WIDTH + 10.0]).unwrap();
						let y = rng.gen_range(-2500.0, -1000.0);
						let vx = rng.gen_range(-128.0, 128.0) + if x < 0.0
						{
							-512.0
						}
						else
						{
							512.0
						};
						let vy = rng.gen_range(-128.0, 128.0);
						let dollar = new_dollar(game_id, obj.dollar_spawn_color, x, y, vx, vy, state);
						state.add_object(dollar);
					}
				}
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
			state.remove_object(game_id);
			let advance = objects.get(player_id).map_or(0, |_| 1);
			start_stage(stage + advance, state);
		}
	}
}

simple_behavior!
{
	GameDraw[obj.is_game] |obj, state|
	{
		let time_left = DURATION - (state.time - obj.start_time);
		let scale = state.disp.get_width() as f32 / WIDTH;
		
		if time_left.abs() < 1.0
		{
			let a = 1.0 - time_left.abs() / 1.0;
			let c = Color::from_rgba_f(a, a, a, a);
			state.prim.draw_filled_rectangle(-WIDTH / 2.0, 0.0, WIDTH / 2.0, -DEATH, c);
		}
		
		if time_left < -1.0
		{
			let mut a = -(time_left + 1.0);
			if a > 1.0
			{
				a = 1.0;
			}
			let c = Color::from_rgba_f(0.0, 0.0, 0.0, a);
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
		let time_text = format!("TIME {}", time_left.ceil() as i32);
		let stage_text = format!("STAGE {}", obj.stage);
		
		let y = state.disp.get_height() as f32 / scale;
		let x1 = -WIDTH / 2.0 + 50.0;
		let x2 = WIDTH / 2.0 - 50.0;
		
		let c = Color::from_rgba(128, 128, 128, 128);
		state.core.draw_text(&state.ui_font, c, 0.0, -y + 50.0, FontAlign::Centre, "POLYTREE");
		state.core.draw_text(&state.ui_font, c, x1, -y + 50.0, FontAlign::Left, &stage_text);
		state.core.draw_text(&state.ui_font, c, x2, -y + 50.0, FontAlign::Right, &time_text);
	}
}
