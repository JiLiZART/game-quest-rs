use crate::place::Place;
use crate::game_item::GameItem;
use std::rc::Rc;


pub struct Player<'a> {
    pub name: Box<&'a str>,
    pub place: Option<Rc<Place<'a>>>,
    pub has_inventory: bool,
    inventory: Option<Box<Vec<GameItem>>>
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: Box::new(name),
            place: None,
            has_inventory: false,
            inventory: None,
        }
    }
}
