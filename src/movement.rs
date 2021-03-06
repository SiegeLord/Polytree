// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

simple_behavior!
{
	OldPos[obj.has_pos] |obj, _state|
	{
		obj.old_x = obj.x;
		obj.old_y = obj.y;
	}
}

simple_behavior!
{
	Movement[obj.can_want_move && obj.has_vel] |obj, _state|
	{
		let a = 512.0;
		obj.ax = 0.0;
		if obj.want_move_left
		{
			obj.ax -= a;
		}
		if obj.want_move_right
		{
			obj.ax += a;
		}
	}
}
