use world::DT;

simple_behavior!
{
	Physics[obj.has_pos && obj.has_vel] |_id, obj, _state|
	{
		obj.x += obj.vx * DT;
		obj.y += obj.vy * DT;
	}
}
