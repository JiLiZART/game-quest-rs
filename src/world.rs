use crate::player::Player;
use crate::place::Place;
use crate::action::Action;

pub struct World<'a> {
    player: Player<'a>,

    // place_hall: &'a Place<'a>,
    pub place_kitchen: Place<'static>,
    // place_room: &'a Place<'a>,
    // place_street: &'a Place<'a>,
    // place_home: &'a Place<'a>,

    actions: Vec<Action<'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        let mut player = Player::new();
        let place_kitchen = Place::new("кухня", "кухня, ничего интересного.", "ты находишься на кухне, на столе чай, надо собрать рюкзак и идти в универ. ");
        // let place_kitchen = Box::new(place_kitchen);
        let actions = vec![Action::new("осмотреться")];

        let world = Self {
            player,
            // place_hall: &Place::new("коридор", "ничего интересного.", ""),
            place_kitchen,
            // place_room: &Place::new("комната", "ты в своей комнате.", ""),
            // place_street: &Place::new("улица", "на улице весна.", ""),
            // place_home: &Place::new("домой", "вот и дома.", ""),
            actions
        };

        player.place = Some(&world.place_kitchen);

        world
    }

    pub fn handle_action(&self, action: &str) -> String {
        let actions = &self.actions;
        let mut actions_iter = actions.into_iter();
        let action = actions_iter.find(| &item| item.name == action);

        return match action {
            None => {
                "неизвестная команда".to_string()
            },
            Some(item) => {
                item.execute()
            }
        }
    }
}

