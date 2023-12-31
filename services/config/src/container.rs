pub struct Container {}

impl Container {
    fn new() -> Self {
        Container {}
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
