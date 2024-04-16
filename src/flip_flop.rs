/// A flip-flopper that keeps track of flip or flop.
#[derive(Debug)]
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

    /// Returns `true` if the states were able to be unsynchronized, returns `false` if they
    /// already were.
    pub fn unsync(&mut self, other: &Self) -> bool {
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

