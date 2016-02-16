// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

#![feature(recover)]

#[macro_use]
extern crate allegro;
extern crate allegro_sys;
extern crate allegro_dialog;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate allegro_ttf;
extern crate allegro_image;
extern crate fern;
#[macro_use]
extern crate log;
extern crate time;
extern crate rand;

#[macro_use]
mod game_state;
mod debug_draw;
mod physics;
mod game;
mod player;
mod movement;
mod branch;
mod dollar;
mod boss;
mod engine;
mod parent;

use debug_draw::*;
use physics::*;
use engine::world::*;
use game_state::*;
use game::*;
use movement::*;
use player::*;
use branch::*;
use dollar::*;
use boss::*;
use parent::*;

use std::fs::OpenOptions;

use allegro::*;
use allegro_dialog::*;
use allegro_primitives::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_image::*;

fn game()
{
	let mut logfile_options = OpenOptions::new();
	logfile_options.write(true).create(true).truncate(true);
	let logger_config = fern::DispatchConfig
	{
		format: Box::new(|msg: &str, level: &log::LogLevel, loc: &log::LogLocation| {
			format!("{} {} {: <24}   {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(),
				level, format!("{}:{}", loc.file(), loc.line()), msg)
		}),
		output: vec![fern::OutputConfig::stderr(), fern::OutputConfig::file_with_options("game.log", &logfile_options),],
		level: log::LogLevelFilter::Trace,
	};
	fern::init_global_logger(logger_config, log::LogLevelFilter::Trace).unwrap();

	info!("It's time to play!");
	
	let mut core = Core::init().unwrap();
	core.install_keyboard().unwrap();
	
	let prim = PrimitivesAddon::init(&core).unwrap();
	let _image = ImageAddon::init(&core).unwrap();
	let font = FontAddon::init(&core).unwrap();
	let ttf = TtfAddon::init(&font).unwrap();
	//~ core.set_new_display_flags(RESIZABLE);
	core.set_new_display_flags(FULLSCREEN_WINDOW);
	core.set_new_display_option(DisplayOption::SampleBuffers, 1, DisplayOptionImportance::Suggest);
	core.set_new_display_option(DisplayOption::Samples, 8, DisplayOptionImportance::Suggest);
	core.set_new_display_option(DisplayOption::Vsync, 1, DisplayOptionImportance::Suggest);
	let disp = Display::new(&core, 1280, 960).unwrap();
	
	core.set_new_bitmap_flags(MAG_LINEAR | MIN_LINEAR);

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
	let mut q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source());
	q.register_event_source(timer.get_event_source());

	let state = GameState::new(core, prim, disp, ttf);
	let mut world = World::<Object, GameState>::new(state);
	
	world.add_logic_behavior(Box::new(OldPos));
	world.add_logic_behavior(Box::new(Physics::new()));
	world.add_logic_behavior(Box::new(GameLogic));
	world.add_logic_behavior(Box::new(Movement));
	world.add_logic_behavior(Box::new(BranchLogic));
	world.add_logic_behavior(Box::new(Gravity));
	world.add_logic_behavior(Box::new(DollarLogic));
	world.add_logic_behavior(Box::new(BossLogic));
	// Must be last.
	world.add_logic_behavior(Box::new(ParentLogic));
	
	world.add_input_behavior(Box::new(GameInput));
	world.add_input_behavior(Box::new(PlayerInput));
	
	world.add_draw_behavior(Box::new(DebugDraw));
	world.add_draw_behavior(Box::new(BranchDraw));
	world.add_draw_behavior(Box::new(SpriteDraw));
	world.add_draw_behavior(Box::new(GameDraw));
	
	start_stage(1, &mut world.state);
	timer.start();
	let offset = world.state.core.get_time() as f32;
	'exit: loop
	{
		for event in &mut q
		{
			world.state.key_down = None;
			world.state.key_up = None;
			
			match event
			{
				DisplayClose{..} =>
				{
					break 'exit;
				},
				DisplayResize{..} =>
				{
					world.state.disp.acknowledge_resize().ok();
				},
				KeyDown{keycode: k, ..} =>
				{
					world.state.key_down = Some(k);
					world.input();
				},
				KeyUp{keycode: k, ..} =>
				{
					world.state.key_up = Some(k);
					world.input();
				},
				TimerTick{..} =>
				{
					if !world.state.paused
					{
						world.state.time += DT;
						world.logic();
					}
					if world.state.quit
					{
						break 'exit;
					}
				},
				_ => ()
			}
		}

		let cur_time = world.state.core.get_time() as f32;
		world.state.draw_interp = ((cur_time - offset - world.state.time) / DT) as f32;
		world.state.core.clear_to_color(Color::from_rgb(0, 0, 0));
		world.draw();
		world.state.core.flip_display();
	}

	info!("All's well that ends well.");
}

allegro_main!
{
	use std::panic::recover;

	match recover(game)
	{
		Err(e) =>
		{
			let err: String = e.downcast_ref::<&'static str>().map(|&e| { e.to_owned()}).or_else(||
			{
				e.downcast_ref::<String>().map(|e| e.clone())
			}).unwrap_or("Unknown error!".to_owned());

			show_native_message_box(None, "Error!", "An error has occurred! See game.log for more info.", &err, Some("You make me sad."), MESSAGEBOX_ERROR);
		}
		Ok(_) => ()
	}
}
