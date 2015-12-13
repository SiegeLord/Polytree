use world::{Object, WorldState};

use allegro::*;

pub fn new_dollar(parent: usize, color: Color, x: f32, y: f32, vx: f32, vy: f32, state: &WorldState) -> Object
{
	Object
	{
		is_dollar: true,
		affected_by_gravity: true,
		is_solid: true,
		size: 14.0,
		has_pos: true,
		x: x,
		y: y,
		has_vel: true,
		vx: vx,
		vy: vy,
		parent: parent,
		sprite: Some(state.dollar.clone()),
		color: color,
		..Object::new()
	}
}

pub struct DollarLogic;

impl ::world::Behavior<::world::Object, ::world::WorldState> for DollarLogic
{
	fn check_object(&self, obj: &::world::Object) -> bool
	{
		obj.is_dollar
	}
	
	fn handle_objects(&mut self, objects: &mut ::id_map::IdMap<::world::Object>, state: &mut ::world::WorldState)
	{
		let mut player_id = 0;
		for &(id, ref obj) in objects.elems()
		{
			if obj.is_player
			{
				player_id = id;
				break;
			}
		}
		if player_id == 0
		{
			return;
		}
		let (player_x, player_y, player_size) =
		{
			let player = objects.get(player_id).unwrap();
			(player.x, player.y, player.size)
		};
		
		let mut collided = false;
		for &mut (id, ref mut obj) in objects.elems_mut()
		{
			if self.check_object(obj)
			{
				let dx = obj.x - player_x;
				let dy = obj.y - player_y;
				let r = obj.size + player_size;
				if dx * dx + dy * dy < r * r
				{
					state.remove_object(id);
					collided = true;
				}
			}
		}
		if collided
		{
			let player = objects.get_mut(player_id).unwrap();
			
			player.y += 64.0;
			player.vx = 0.0;
			player.vy = 64.0;
			if player.y > 0.0
			{
				player.y = 0.0;
			}
		}
	}
}


pub struct SpriteDraw;

impl ::world::Behavior<::world::Object, ::world::WorldState> for SpriteDraw
{
	fn check_object(&self, obj: &::world::Object) -> bool
	{
		obj.sprite.is_some()
	}
	
	fn handle_objects(&mut self, objects: &mut ::id_map::IdMap<::world::Object>, state: &mut ::world::WorldState)
	{
		state.core.hold_bitmap_drawing(true);
		for &(_, ref obj) in objects.elems()
		{
			if self.check_object(obj)
			{
				let sprite = obj.sprite.as_ref().unwrap();
				let bw = sprite.get_width() as f32;
				let bh = sprite.get_height() as f32;
				let sw = obj.size;
				let sh = bh * sw / bw;
				state.core.draw_tinted_scaled_bitmap(&**sprite, obj.color, 0.0, 0.0, bw, bh, obj.x - sw, obj.y - sh, 2.0 * sw, 2.0 * sw, BitmapDrawingFlags::zero());
			}
		}
		state.core.hold_bitmap_drawing(false);
	}
}
