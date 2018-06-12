pub enum Action {
    List,
    Clone,
    Pull,
    Checkout,
}

impl Action {
    pub fn from(input: &str) -> Result<Action, &str> {
        let action = match input {
            "list" => Action::List,
            "clone" => Action::Clone,
            "pull" => Action::Pull,
            "checkout" => Action::Checkout,
            _ => return Err(input)
        };

        Ok(action)
    }
}
