simple_behavior!
{
	Movement[obj.can_want_move && obj.has_vel] |_id, obj, _state|
	{
		let vel = 128.0;
		obj.vx = 0.0;
		obj.vy = 0.0;
		if obj.want_move_left
		{
			obj.vx -= vel;
		}
		if obj.want_move_right
		{
			obj.vx += vel;
		}
	}
}
