#![feature(catch_panic)]

#[macro_use]
extern crate allegro;
extern crate fern;
#[macro_use]
extern crate log;
extern crate time;
extern crate allegro_dialog;

mod id_map;
mod world;

use allegro::*;
use allegro_dialog::*;

static DT: f32 = 1.0 / 60.0;

fn game()
{
	let logger_config = fern::DispatchConfig
	{
		format: Box::new(|msg: &str, level: &log::LogLevel, loc: &log::LogLocation| {
			format!("{} {} {}:{}  {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(),
				level, loc.module_path(), loc.line(), msg)
		}),
		output: vec![fern::OutputConfig::stderr()],
		level: log::LogLevelFilter::Trace,
	};
	fern::init_global_logger(logger_config, log::LogLevelFilter::Trace).unwrap();

	info!("It's time to play!");
	
	let mut core = Core::init().unwrap();
	core.install_keyboard().unwrap();
	
	//~ let prim = PrimitivesAddon::init(&core).unwrap();
	//~ let _image = ImageAddon::init(&core).unwrap();
	//~ let audio = AudioAddon::init(&core).unwrap();
	//~ let _acodec = AcodecAddon::init(&audio).unwrap();
	//~ let font = FontAddon::init(&core).unwrap();
	core.set_new_display_flags(RESIZABLE);
	let disp = Display::new(&core, 1280, 960).unwrap();

	let timer = Timer::new(&core, DT as f64).unwrap();
	let mut q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source());
	q.register_event_source(timer.get_event_source());


	timer.start();
	'exit: loop
	{
		for event in &mut q
		{
			//~ world.state.key_down = None;
			//~ world.state.key_up = None;
			//~ world.state.key_char = None;
			
			match event
			{
				DisplayClose{..} =>
				{
					break 'exit;
				},
				DisplayResize{..} =>
				{
					disp.acknowledge_resize().ok();
				},
				//~ KeyDown{keycode: k, ..} =>
				//~ {
					//~ world.state.key_down = Some(k);
					//~ world.input();
				//~ },
				//~ KeyChar{unichar: c, ..} =>
				//~ {
					//~ world.state.key_char = Some(c);
					//~ world.input();
				//~ },
				//~ KeyUp{keycode: k, ..} =>
				//~ {
					//~ world.state.key_up = Some(k);
					//~ world.input();
				//~ },
				TimerTick{..} =>
				{
					//~ if !world.state.paused
					//~ {
						//~ world.state.time += DT;
						//~ world.logic();
					//~ }
					//~ if world.state.quit
					//~ {
						//~ break 'exit;
					//~ }
				},
				_ => ()
			}
		}

		core.clear_to_color(core.map_rgb(0, 0, 0));
		core.flip_display();
	}

	info!("All's well that ends well.");
}

allegro_main!
{
	use std::thread::catch_panic;

	match catch_panic(game)
	{
		Err(e) =>
		{
			let err: String = e.downcast_ref::<&'static str>().map(|&e| { e.to_owned()}).or_else(||
			{
				e.downcast_ref::<String>().map(|e| e.clone())
			}).unwrap_or("Unknown error!".to_owned());

			show_native_message_box(None, "Error!", "An error has occurred! Redirect stderr from the command line for more info.", &err, Some("You make me sad."), MESSAGEBOX_ERROR);
		}
		Ok(_) => ()
	}
}
