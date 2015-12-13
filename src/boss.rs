use world::{WorldState, random_color, Object, DEATH, DT, BOSS_RX, BOSS_RY, BOSS_RATE};
use dollar::new_dollar;
use std::f32::consts::PI;
use rand::{self, Rng};
use allegro::*;

pub fn new_boss(parent: usize, dollar_spawn_color: Color, state: &WorldState) -> Object
{
	Object
	{
		is_boss: true,
		has_pos: true,
		x: BOSS_RX,
		y: -DEATH,
		parent: parent,
		start_time: state.time,
		sprite: Some(state.boss.clone()),
		color: random_color(&state.core),
		size: 32.0,
		dollar_spawn_color: dollar_spawn_color,
		..Object::new()
	}
}

simple_behavior!
{
	BossLogic[obj.is_boss] |_id, obj, state|
	{
		let theta = 2.0 * PI * (state.time - obj.start_time) / BOSS_RATE;
		obj.x = BOSS_RX * theta.cos();
		obj.y = BOSS_RY * theta.sin() - DEATH;
		
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < 0.2 * DT
		{
			let n = 16;
			for i in 0..n
			{
				let theta = 2.0 * PI * (i as f32) / (n as f32);
				let dollar = new_dollar(obj.parent, obj.dollar_spawn_color, obj.x, obj.y, theta.cos() * 256.0, theta.sin() * 256.0, state);
				state.add_object(dollar);
			}
		}
	}
}

