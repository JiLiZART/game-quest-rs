use crate::action::Action;
use crate::world::World;
use std::rc::Rc;

pub struct ActionLookAround<'a> {
    pub name: &'a str,
}

impl<'a> ActionLookAround<'a> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            name: "осмотреться",
        })
    }
}

impl<'a> Action for ActionLookAround<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn execute(&self, world: &World, _: Vec<&str>) -> String {
        return world.player.place.look_around().to_string();
    }
}
