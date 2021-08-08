use crate::player::Player;
use crate::place::Place;
use crate::action::Action;
use crate::action_look_around::ActionLookAround;

use std::rc::Rc;
use std::cell::RefCell;

type Actions = Vec<Box<dyn Action>>;

pub struct World {
    pub player: Player<'static>,

    pub place_hall: Rc<Place<'static>>,
    pub place_kitchen: Rc<Place<'static>>,
    // place_room: &'a Place<'a>,
    // place_street: &'a Place<'a>,
    // place_home: &'a Place<'a>,

    pub actions: Actions,
}

impl World {
    pub fn new(player_name: &'static str) -> Self {
        let place_kitchen = Rc::new(
            Place::new("кухня", "кухня, ничего интересного.", "ты находишься на кухне, на столе чай, надо собрать рюкзак и идти в универ. ")
        );

        let mut world = Self {
            player: Player::new(player_name, Rc::clone(&place_kitchen)),
            place_hall: Rc::new(Place::new("коридор", "ничего интересного.", "")),
            place_kitchen: Rc::clone(&place_kitchen),

            // place_room: &Place::new("комната", "ты в своей комнате.", ""),
            // place_street: &Place::new("улица", "на улице весна.", ""),
            // place_home: &Place::new("домой", "вот и дома.", ""),
            actions: vec![],
        };

        world.actions.push(ActionLookAround::new());

        world
    }

    pub fn handle_action(&self, action: &str, args: Vec<&str>) -> String {
        let actions = &self.actions;
        let mut actions_iter = actions.into_iter();
        let action = actions_iter.find(|&item| item.get_name() == action);

        return match action {
            None => {
                "неизвестная команда".to_string()
            }
            Some(item) => {
                item.execute(&self, args)
            }
        };
    }
}

