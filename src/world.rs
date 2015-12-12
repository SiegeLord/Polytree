use id_map::IdMap;

pub struct Object
{
	pub remove_me: bool,
	
	pub has_pos: bool,
	pub x: i32,
}

pub trait Behavior<O, S>
{
	fn check_object(&self, &O) -> bool
	{
		true
	}
	
	fn handle_objects(&mut self, objects: &mut IdMap<O>, state: &mut S);
}

pub struct Movement;

impl Behavior<Object, WorldState> for Movement
{
	fn check_object(&self, obj: &Object) -> bool
	{
		obj.has_pos
	}
	
	fn handle_objects(&mut self, objects: &mut IdMap<Object>, _state: &mut WorldState)
	{
		for &mut (_, ref mut obj) in objects.elems_mut()
		{
			if self.check_object(obj)
			{
				obj.x += 1;
			}
		}
	}
}

pub struct WorldState
{
	pub state: i32,
}

pub struct World
{
	objects: IdMap<Object>,
	logic_behaviors: Vec<Box<Behavior<Object, WorldState>>>,
	state: WorldState,
}

impl World
{
	pub fn logic(&mut self)
	{
		for behavior in &mut self.logic_behaviors
		{
			behavior.handle_objects(&mut self.objects, &mut self.state);
		}
		
		let mut ids_to_remove = vec![];
		for &(id, ref e) in self.objects.elems()
		{
			if e.remove_me
			{
				ids_to_remove.push(id);
			}
		}
		
		for id in ids_to_remove
		{
			self.objects.remove(id);
		}
	}
}
