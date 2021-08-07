use crate::player::Player;
use crate::place::Place;
use crate::action::Action;
use crate::action_look_around::ActionLookAround;

use std::rc::Rc;
use std::cell::RefCell;

pub struct World {
    pub player: Player<'static>,

    // place_hall: &'a Place<'a>,
    pub place_kitchen: Rc<Place<'static>>,
    // place_room: &'a Place<'a>,
    // place_street: &'a Place<'a>,
    // place_home: &'a Place<'a>,

    actions: RefCell<Vec<Box<dyn Action>>>
}

impl World {
    pub fn new(player_name: &'static str) -> Rc<Self> {
        // let place_kitchen = Box::new(place_kitchen);
        let place_kitchen = Rc::new(
            Place::new("кухня", "кухня, ничего интересного.", "ты находишься на кухне, на столе чай, надо собрать рюкзак и идти в универ. ")
        );

        let mut world = Rc::new(
            Self {
                player: Player::new(player_name),
                place_kitchen: Rc::clone(&place_kitchen),
                // place_hall: &Place::new("коридор", "ничего интересного.", ""),

                // place_room: &Place::new("комната", "ты в своей комнате.", ""),
                // place_street: &Place::new("улица", "на улице весна.", ""),
                // place_home: &Place::new("домой", "вот и дома.", ""),
                actions: RefCell::new(vec![]),
            }
        );

        let mut action_world = Rc::clone(&world);
        let action_look_around = Box::new(ActionLookAround::new(action_world));

        world.player.place = Some(Rc::clone(&place_kitchen));
        world.actions = RefCell::new(vec![action_look_around]);

        Rc::clone(&world)
    }

    pub fn handle_action(&self, action: &str, args: Vec<&str>) -> String {
        let actions = &self.actions;
        let mut actions_iter = actions.into_iter();
        let action = actions_iter.find(| &item| item.get_name() == action);

        return match action {
            None => {
                "неизвестная команда".to_string()
            },
            Some(item) => {
                item.execute(args)
            }
        }
    }
}

