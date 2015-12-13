use world::{DT, Object};

use allegro::*;
use rand::{self, Rng};

pub fn new_branch(parent: usize, color: Color, sx: f32, sy: f32, dx: f32, dy: f32, t: f32) -> Object
{
	let mut rng = rand::thread_rng();
	Object
	{
		is_branch: true,
		branch_start_x: sx,
		branch_start_y: sy,
		branch_dir_x: dx,
		branch_dir_y: dy,
		branch_start_time: t,
		branch_spawns: 1,
		branch_max_dur: rng.gen_range(0.25, 2.1),
		color: color,
		parent: parent,
		..Object::new()
	}
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
	BranchLogic[obj.is_branch] |_id, obj, state|
	{
		let mut rng = rand::thread_rng();
		if rng.gen::<f32>() < 0.9 * DT && obj.branch_spawns > 0 && get_branch_dur(&obj, state.time) > 0.2
		{
			let dt = get_branch_dur(&obj, state.time) * rng.gen_range(0.25, 0.75);
			let spawn_x = dt * obj.branch_dir_x + obj.branch_start_x;
			let spawn_y = dt * obj.branch_dir_y + obj.branch_start_y;
			
			let time = state.time;
			state.add_object(new_branch(obj.parent, obj.color, spawn_x, spawn_y, -obj.branch_dir_x, obj.branch_dir_y, time));
			obj.branch_spawns -= 1;
		}
		obj.branch_start_y += 48.0 * DT;
	}
}

simple_behavior!
{
	BranchDraw[obj.is_branch] |_id, obj, state|
	{
		let dt = get_branch_dur(&obj, state.time);
		let end_x = dt * obj.branch_dir_x + obj.branch_start_x;
		let end_y = dt * obj.branch_dir_y + obj.branch_start_y;
		
		let alpha = 1.0 - 0.5 * dt / obj.branch_max_dur;
		let (r, g, b) = obj.color.unmap_rgb_f();
		let c = state.core.map_rgba_f(r * alpha, g * alpha, b * alpha, alpha);
		
		state.prim.draw_line(obj.branch_start_x, obj.branch_start_y, end_x, end_y, c, 10.0);
	}
}
