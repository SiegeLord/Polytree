use branch::get_branch_end;
use world::DT;

simple_behavior!
{
	Gravity[obj.affected_by_gravity] |_id, obj, _state|
	{
		obj.vy += 8.0 * DT;
	}
}

pub fn segment_intersect(s1: (f32, f32, f32, f32), s2: (f32, f32, f32, f32)) -> Option<(f32, f32)>
{
	let ((x1, y1, x2, y2), (x3, y3, x4, y4)) = (s1, s2);
	let d = (y4 - y3) * (x2 - x2) - (x4 - x3) * (y2 - y1);
	let n1 = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
	let n2 = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
	if d == 0.0
	{
		None
	}
	else
	{
		let u1 = n1 / d;
		let u2 = n2 / d;
		if u1 >= 0.0 && u1 <= 1.0 && u2 >= 0.0 && u2 <= 1.0
		{
			Some((u1, u2))
		}
		else
		{
			None
		}
	}
}

pub fn segment_distance(segment: (f32, f32, f32, f32), px: f32, py: f32) -> f32
{
	let dx = segment.2 - segment.0;
	let dy = segment.3 - segment.1;
	
	let pdx = px - segment.0;
	let pdy = py - segment.1;
	
	let len_sq = dx * dx + dy * dy;

	let mut u = (dx * pdx + dy * pdy) / len_sq;
	if u < 0.0
	{
		u = 0.0;
	}
	else if u > 1.0
	{
		u = 1.0;
	}
	let ix = segment.0 + dx * u;
	let iy = segment.1 + dy * u;
	
	let idx = px - ix;
	let idy = py - iy;
	(idx * idx + idy * idy).sqrt()
}

pub struct Physics
{
	branches: Vec<(f32, f32, f32, f32)>,
}

impl Physics
{
	pub fn new() -> Physics
	{
		Physics
		{
			branches: vec![],
		}
	}
}
		
impl ::world::Behavior<::world::Object, ::world::WorldState> for Physics
{
	fn check_object(&self, obj: &::world::Object) -> bool
	{
		true
	}
	
	fn handle_objects(&mut self, objects: &mut ::id_map::IdMap<::world::Object>, state: &mut ::world::WorldState)
	{
		self.branches.clear();
		for &(_, ref obj) in objects.elems()
		{
			if obj.is_branch
			{
				let end = get_branch_end(obj, state.time);
				self.branches.push((obj.branch_start_x, obj.branch_start_y, end.0, end.1));
			}
		}
		
		for &mut (_, ref mut obj) in objects.elems_mut()
		{
			if obj.has_vel && obj.has_pos
			{
				obj.vx += obj.ax * DT;
				obj.vy += obj.ay * DT;
				let mut best_vx = obj.vx;
				let mut best_vy = obj.vy;
				let mut best_nx = obj.x + obj.vx * DT;
				let mut best_ny = obj.y + obj.vy * DT;
				if obj.is_solid && (obj.vx != 0.0 || obj.vy != 0.0)
				{
					let mut collided = false;
					
					for &branch in &self.branches
					{
						if segment_distance(branch, obj.x, obj.y) < 4.0
						{
							collided = true;
							let bdx = branch.2 - branch.0;
							let bdy = branch.3 - branch.1;
							let branch_len_sq = bdx * bdx + bdy * bdy;
							let dir = (bdx * obj.vx + bdy * obj.vy) / branch_len_sq;
							let vx = dir * bdx;
							let vy = dir * bdy;
							let nx = obj.x + DT * vx;
							let ny = obj.y + DT * vy;
							if !collided || ny < best_ny
							{
								best_nx = nx;
								best_ny = ny;
								best_vx = vx;
								best_vy = vy;
							}
						}
					}
					
				}
				obj.vx = best_vx;
				obj.vy = best_vy;
				obj.x = best_nx;
				obj.y = best_ny;
			}
		}
	}
}
