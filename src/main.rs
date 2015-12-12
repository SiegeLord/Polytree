extern crate allegro;

mod id_map;
mod world;

use allegro::*;

fn main()
{
	let core = Core::init().unwrap();
	
	let display = Display::new(&core, 800, 600);
	
	core.clear_to_color(core.map_rgb(0, 0, 0));
	core.flip_display();
	core.rest(1.0);
}
