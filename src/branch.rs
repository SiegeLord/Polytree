// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use world::{DT, Object};
use id_map::UniqueId;

use allegro::*;
use rand::{self, Rng};

pub fn new_branch(parent: usize, color: Color, sx: f32, sy: f32, dx: f32, dy: f32, t: f32, id: UniqueId) -> Object
{
	let mut rng = rand::thread_rng();
	let mut obj = Object::new(id);
	obj.is_branch = true;
	obj.branch_start_x = sx;
	obj.branch_start_y = sy;
	obj.branch_dir_x = dx;
	obj.branch_dir_y = dy;
	obj.branch_start_time = t;
	obj.branch_spawns = 1;
	obj.branch_max_dur = rng.gen_range(0.25, 2.1);
	obj.color = color;
	obj.parent = parent;
	obj
}

pub fn get_branch_end(obj: &Object, cur_time: f32) -> (f32, f32)
{
	let dt = get_branch_dur(obj, cur_time);
	(dt * obj.branch_dir_x + obj.branch_start_x,
	 dt * obj.branch_dir_y + obj.branch_start_y)
}

pub fn get_branch_dur(obj: &Object, cur_time: f32) -> f32
{
	let dt = cur_time - obj.branch_start_time;
	if dt > obj.branch_max_dur
	{
		obj.branch_max_dur
	}
	else
	{
		dt
	}
}

simple_behavior!
{
	BranchLogic[obj.is_branch] |obj, state|
	{
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < 0.9 * DT && obj.branch_spawns > 0 && get_branch_dur(&obj, state.time) > 0.2
		{
			let dt = get_branch_dur(&obj, state.time) * rng.gen_range(0.25, 0.75);
			let spawn_x = dt * obj.branch_dir_x + obj.branch_start_x;
			let spawn_y = dt * obj.branch_dir_y + obj.branch_start_y;
			
			let time = state.time;
			let branch = new_branch(obj.parent, obj.color, spawn_x, spawn_y, -obj.branch_dir_x, obj.branch_dir_y, time, state.new_id());
			state.add_object(branch);
			obj.branch_spawns -= 1;
		}
		obj.branch_start_y += 48.0 * DT;
	}
}

simple_behavior!
{
	BranchDraw[obj.is_branch] |obj, state|
	{
		let dt = get_branch_dur(&obj, state.time);
		let end_x = dt * obj.branch_dir_x + obj.branch_start_x;
		let end_y = dt * obj.branch_dir_y + obj.branch_start_y;
		
		let alpha = 1.0 - 0.5 * dt / obj.branch_max_dur;
		let (r, g, b) = obj.color.to_rgb_f();
		let c = Color::from_rgba_f(r * alpha, g * alpha, b * alpha, alpha);
		
		state.prim.draw_line(obj.branch_start_x, obj.branch_start_y, end_x, end_y, c, 10.0);
	}
}
