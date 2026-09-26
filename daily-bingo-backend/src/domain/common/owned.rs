pub struct Owned<O, R> {
    pub owner: O,
    pub resource: R,
}

impl<O, R> Owned<O, R> {
    pub fn new(owner: O, resource: R) -> Self {
        Self { owner, resource }
    }
}
