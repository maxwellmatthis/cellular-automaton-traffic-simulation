/// A flip-flopper that keeps track of flip or flop.
pub struct FlipFlop {
    state: bool
}

impl FlipFlop {
    /// Create a flip from with a state of `true`.
    pub fn new() -> Self {
        Self { state: true }
    }

    /// Flips the state.
    pub fn flip_flop(&mut self) {
        self.state = !self.state;
    }

    /// Checks if the states of two flip flops match. If they do, take `self` out of sync.
    pub fn sync(&mut self, other: &Self) -> bool {
        if other.state() == self.state() {
            self.flip_flop();
            return true;
        }
        false
    }

    /// Returns the state of the flip flow.
    pub fn state(&self) -> bool {
        self.state
    }
}

