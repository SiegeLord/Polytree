simple_behavior!
{
	DebugDraw[obj.debug_draw && obj.has_pos] |_id, obj, state|
	{
		state.prim.draw_circle(obj.x, obj.y, 10.0, state.core.map_rgb(64, 255, 255), 4.0);
	}
}

