use crate::game_item::GameItem;

pub struct PlaceItem<'a> {
    name: String,
    description: String,
    game_items: &'a Vec<GameItem>,
}
