use world::{DT, Object};

use rand::{self, Rng};

pub fn new_branch(sx: f32, sy: f32, dx: f32, dy: f32, t: f32) -> Object
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
		branch_max_dur: rng.gen_range(1.0, 5.0),
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
		if rng.gen::<f32>() < 0.65 * DT && obj.branch_spawns > 0 && get_branch_dur(&obj, state.time) > 0.5
		{
			let dt = get_branch_dur(&obj, state.time) * rng.gen_range(0.25, 0.75);
			let spawn_x = dt * obj.branch_dir_x + obj.branch_start_x;
			let spawn_y = dt * obj.branch_dir_y + obj.branch_start_y;
			
			let time = state.time;
			state.add_object(new_branch(spawn_x, spawn_y, -obj.branch_dir_x, obj.branch_dir_y, time));
			obj.branch_spawns -= 1;
			info!("Spawns left: {}", obj.branch_spawns);
		}
		obj.branch_start_y += 16.0 * DT;
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
		let c = state.core.map_rgba_f(0.5 * alpha, 1.0 * alpha, 0.8 * alpha, alpha);
		
		state.prim.draw_line(obj.branch_start_x, obj.branch_start_y, end_x, end_y, c, 10.0);
	}
}
