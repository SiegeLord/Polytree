// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use world::{WorldState, random_color, Object, DEATH, DT, BOSS_RX, BOSS_RY, BOSS_RATE};
use dollar::new_dollar;
use std::f32::consts::PI;
use rand::{self, Rng};
use allegro::*;

pub fn new_boss(parent: usize, dollar_spawn_color: Color, state: &mut WorldState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_boss = true;
	obj.has_pos = true;
	obj.x = BOSS_RX;
	obj.y = -DEATH;
	obj.parent = parent;
	obj.start_time = state.time;
	obj.sprite = Some(state.boss.clone());
	obj.color = random_color();
	obj.size = 32.0;
	obj.dollar_spawn_color = dollar_spawn_color;
	obj
}

simple_behavior!
{
	BossLogic[obj.is_boss] |obj, state|
	{
		let theta = 2.0 * PI * (state.time - obj.start_time) / BOSS_RATE;
		obj.x = BOSS_RX * theta.cos();
		obj.y = BOSS_RY * theta.sin() - DEATH;
		
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < 0.2 * DT
		{
			let n = 13;
			for i in 0..n
			{
				let theta = 2.0 * PI * (i as f32) / (n as f32);
				let dollar = new_dollar(obj.parent, obj.dollar_spawn_color, obj.x, obj.y, theta.cos() * 256.0, theta.sin() * 256.0, state);
				state.add_object(dollar);
			}
		}
	}
}

