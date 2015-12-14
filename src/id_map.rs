// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

#![allow(dead_code)]

use std::collections::HashMap;

pub struct IdMap<T>
{
	// id, element
	elems: Vec<(usize, T)>,
	id_to_idx: HashMap<usize, usize>,
	next_id: usize,
}

impl<T> IdMap<T>
{
	pub fn new() -> IdMap<T>
	{
		IdMap
		{
			elems: vec![],
			id_to_idx: HashMap::new(),
			next_id: 1,
		}
	}

	pub fn insert(&mut self, e: T) -> usize
	{
		let id = self.next_id;
		self.id_to_idx.insert(id, self.elems.len());
		self.elems.push((id, e));
		self.next_id += 1;
		id
	}

	pub fn remove(&mut self, id: usize)
	{
		let idx = self.id_to_idx[&id];
		// This element will be moved to the idx.
		*self.id_to_idx.get_mut(&self.elems.last().unwrap().0).unwrap() = idx;
		self.elems.swap_remove(idx);
		self.id_to_idx.remove(&id);
	}

	pub fn get(&self, id: usize) -> Option<&T>
	{
		self.id_to_idx.get(&id).map(|&idx| &self.elems[idx].1)
	}

	pub fn get_mut(&mut self, id: usize) -> Option<&mut T>
	{
		match self.id_to_idx.get(&id)
		{
			Some(&idx) => Some(&mut self.elems[idx].1),
			None => None
		}
	}

	pub fn len(&self) -> usize
	{
		self.elems.len()
	}

	pub fn elems(&self) -> &[(usize, T)]
	{
		&self.elems
	}

	pub fn elems_mut(&mut self) -> &mut [(usize, T)]
	{
		&mut self.elems
	}
	
	pub fn next_id(&self) -> usize
	{
		self.next_id
	}
}

#[test]
fn basic()
{
	let mut map = IdMap::<i32>::new();
	let id1 = map.insert(1);
	let id2 = map.insert(2);
	assert_eq!(1, *map.get(id1).unwrap());
	assert_eq!(2, *map.get(id2).unwrap());
	assert_eq!(2, map.len());
	map.remove(id1);
	assert_eq!(2, *map.get(id2).unwrap());
	assert_eq!(1, map.len());
	let id3 = map.insert(3);
	assert_eq!(2, *map.get(id2).unwrap());
	assert_eq!(3, *map.get(id3).unwrap());
	assert_eq!(2, map.len());
}
