pub struct Poker(pub Vec<u8>);

impl Poker {
    pub fn new(cards: Vec<u8>) -> Poker {
        Poker(cards)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn check(&self) -> (u8, u8) {
        let first = self.0[0];
        let second = self.0[1];
        (first, second)
    }

    pub fn swap(&mut self) {
        let (first, second) = self.check();
        self.0[0] = second;
        self.0[1] = first;
    }

    pub fn sink(&mut self) {
        let top = self.0.remove(0);
        self.0.push(top);
    }
}
