use crate::game_item::GameItem;
use crate::place::Place;
use std::rc::Rc;

pub struct Player<'a> {
    pub name: Box<&'a str>,
    pub place: Rc<Place<'a>>,
    pub has_inventory: bool,
    inventory: Option<Box<Vec<GameItem>>>,
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str, place: Rc<Place<'a>>) -> Self {
        Self {
            name: Box::new(name),
            place,
            has_inventory: false,
            inventory: None,
        }
    }
}
