pub enum Action {
    List,
    Clone,
    Pull,
    Checkout,
    MirrorPull,
    MirrorPush,
}

impl Action {
    pub fn from(input: &str) -> Result<Action, &str> {
        let action = match input {
            "list" => Action::List,
            "clone" => Action::Clone,
            "pull" => Action::Pull,
            "checkout" => Action::Checkout,
            "mirror_pull" => Action::MirrorPull,
            "mirror_push" => Action::MirrorPush,
            _ => return Err(input)
        };

        Ok(action)
    }
}
