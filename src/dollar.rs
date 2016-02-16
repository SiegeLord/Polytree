// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use engine::id_map::HasId;
use game_state::{Object, GameState};

use allegro::*;

pub fn new_dollar(parent: usize, color: Color, x: f32, y: f32, vx: f32, vy: f32, state: &mut GameState) -> Object
{
	let mut obj = Object::new(state.new_id());
	obj.is_dollar = true;
	obj.affected_by_gravity = true;
	obj.is_solid = true;
	obj.size = 10.0;
	obj.has_pos = true;
	obj.x = x;
	obj.y = y;
	obj.old_x = x;
	obj.old_y = y;
	obj.has_vel = true;
	obj.vx = vx;
	obj.vy = vy;
	obj.parent = parent;
	obj.sprite = Some(state.bitmap_manager.load(&state.core, "data/dollar.png").unwrap());
	obj.color = color;
	obj
}

pub struct DollarLogic;

impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for DollarLogic
{
	fn check_object(&self, obj: &::game_state::Object) -> bool
	{
		obj.is_dollar
	}
	
	fn handle_objects(&mut self, objects: &mut ::engine::id_map::IdMap<::game_state::Object>, state: &mut ::game_state::GameState)
	{
		let mut player_id = 0;
		for obj in objects.elems()
		{
			if obj.is_player
			{
				player_id = obj.get_id();
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
		for obj in objects.elems_mut()
		{
			if self.check_object(obj)
			{
				let dx = obj.x - player_x;
				let dy = obj.y - player_y;
				let r = obj.size + player_size;
				if dx * dx + dy * dy < r * r
				{
					state.remove_object(obj.get_id());
					collided = true;
				}
				
				if obj.y >= -obj.size
				{
					state.remove_object(obj.get_id());
				}
			}
		}
		if collided
		{
			let player = objects.get_mut(player_id).unwrap();
			
			player.y += 64.0;
			player.vx = 0.0;
			player.vy = 64.0;
			if player.y > -player.size
			{
				player.y = -player.size;
			}
		}
	}
}


pub struct SpriteDraw;

impl ::engine::world::Behavior<::game_state::Object, ::game_state::GameState> for SpriteDraw
{
	fn check_object(&self, obj: &::game_state::Object) -> bool
	{
		obj.sprite.is_some()
	}
	
	fn handle_objects(&mut self, objects: &mut ::engine::id_map::IdMap<::game_state::Object>, state: &mut ::game_state::GameState)
	{
		state.core.hold_bitmap_drawing(true);
		for obj in objects.elems()
		{
			if self.check_object(obj)
			{
				let sprite = obj.sprite.as_ref().unwrap();
				let bw = sprite.get_width() as f32;
				let bh = sprite.get_height() as f32;
				let sw = obj.size;
				let sh = bh * sw / bw;
				let x = obj.old_x + state.draw_interp * (obj.x - obj.old_x);
				let y = obj.old_y + state.draw_interp * (obj.y - obj.old_y);
				state.core.draw_tinted_scaled_bitmap(&**sprite, obj.color, 0.0, 0.0, bw, bh, x - sw, y - sh, 2.0 * sw, 2.0 * sh, BitmapDrawingFlags::zero());
			}
		}
		state.core.hold_bitmap_drawing(false);
	}
}
