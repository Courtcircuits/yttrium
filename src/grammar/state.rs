pub struct State {
    pub isFinal: bool,
    pub uuid: String,
}

pub fn create_state(isFinal: bool) -> State {
    State::new(isFinal)
}

impl State {
    pub fn new(isFinal: bool) -> State {
        State {
            isFinal,
            uuid: uuid::Uuid::new_v4().to_string(),
        }
    }
    pub fn is_final(&self) -> bool {
        self.isFinal
    }
    pub fn equals(&self, other: &State) -> bool {
        self.uuid == other.uuid
    }
}
