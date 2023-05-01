pub struct TetherAgent {
    role: String,
    id: String,
}

impl TetherAgent {
    pub fn new(role: &str, id: Option<&str>) -> Self {
        TetherAgent {
            role: String::from(role),
            id: match id {
                Some(s) => String::from(s),
                None => String::from("any"),
            },
        }
    }

    pub fn set_role(&mut self, role: &str) {
        self.role = String::from(role);
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = String::from(id);
    }
}
