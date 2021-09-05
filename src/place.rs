use crate::door::Door;
use crate::place_item::PlaceItem;

pub struct Place<'a> {
    id: &'a str,
    description: &'a str,
    entering_message: &'a str,
    item_places: Box<Vec<PlaceItem<'a>>>,
    exits: Box<Vec<Door>>,
}

impl Default for Place<'_> {
    fn default() -> Self {
        Place {
            id: "",
            description: "",
            entering_message: "",
            item_places: Box::new(Vec::new()),
            exits: Box::new(Vec::new()),
        }
    }
}

impl<'a> Place<'a> {
    pub fn new(id: &'a str, entering_message: &'a str, description: &'a str) -> Self {
        Self {
            id,
            entering_message,
            description,
            ..Place::default()
        }
    }

    pub fn look_around(&self) -> &'a str {
        return self.description;
    }
}
