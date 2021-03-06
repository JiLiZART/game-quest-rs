use crate::world::World;

pub struct ActionA<'a> {
    pub name: &'a str,
}

pub trait Action {
    fn get_name(&self) -> &str;
    fn execute(&self, _: &World, _: Vec<&str>) -> String {
        format!("Execute action {}", self.get_name())
    }
}
