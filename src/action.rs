pub struct Action<'a> {
    pub name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn execute(&self) -> String {
        format!("Execute action {}", self.name)
    }
}
