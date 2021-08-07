use crate::action::Action;
use crate::world::World;
use std::rc::Rc;

pub struct ActionLookAround<'a> {
    pub name: &'a str,
    pub world: Rc<World>
}

impl<'a> ActionLookAround<'a> {
    pub fn new(world: Rc<World>) -> Self {
        Self {
            name: "осмотреться",
            world
        }
    }
}

impl<'a> Action for ActionLookAround<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn execute(&self, args: Vec<&str>) -> String {
        let place = &self.world.player.place;

        return match place {
            None => {
                "ты нигде".to_string()
            },
            Some(place) => {
                place.look_around().to_string()
            }
        }
    }
}
