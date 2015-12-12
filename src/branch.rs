use world::{DT, Object};

use rand::{self, Rng};

pub fn new_branch(sx: f32, sy: f32, dx: f32, dy: f32, t: f32) -> Object
{
	let mut rng = rand::thread_rng();
	let mut branch = Object::new();
	branch.is_branch = true;
	branch.branch_start_x = sx;
	branch.branch_start_y = sy;
	branch.branch_dir_x = dx;
	branch.branch_dir_y = dy;
	branch.branch_start_time = t;
	branch.branch_max_dur = rng.gen_range(1.0, 5.0);
	branch
}

fn get_branch_dur(obj: &Object, time: f32) -> f32
{
	let dt = time - obj.branch_start_time;
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
		if rng.gen::<f32>() < 0.25 * DT
		{
			let dt = get_branch_dur(&obj, state.time) * rng.gen_range(0.25, 0.75);
			let spawn_x = dt * obj.branch_dir_x + obj.branch_start_x;
			let spawn_y = dt * obj.branch_dir_y + obj.branch_start_y;
			
			let time = state.time;
			state.add_object(new_branch(spawn_x, spawn_y, -obj.branch_dir_x, obj.branch_dir_y, time));
		}
	}
}

simple_behavior!
{
	BranchDraw[obj.is_branch] |_id, obj, state|
	{
		let dt = get_branch_dur(&obj, state.time);
		let end_x = dt * obj.branch_dir_x + obj.branch_start_x;
		let end_y = dt * obj.branch_dir_y + obj.branch_start_y;
		
		let alpha = 1.0 - dt / obj.branch_max_dur;
		let c = state.core.map_rgba_f(0.5 * alpha, 1.0 * alpha, 0.8 * alpha, alpha);
		
		state.prim.draw_line(obj.branch_start_x, obj.branch_start_y, end_x, end_y, c, 10.0);
	}
}
