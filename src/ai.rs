
#[deriving(Clone)]
pub enum Strategy {
    Minimax,
    LowestAvailable
}

pub struct AI {
    strategy: Strategy
}

impl AI {
    pub fn new(strategy: Strategy) -> AI {
        AI { strategy: strategy }
    }

    pub fn get_move(&self, spaces: ~[char]) -> Option<int> {
        ::std::rt::io::timer::sleep(1000); // simulate thinking

        match (*self).strategy {
            LowestAvailable => self.get_lowest_available_index(spaces),
            Minimax         => self.minimax(spaces)
        }
    }

    fn get_lowest_available_index(&self, spaces: ~[char]) -> Option<int> {
        let empty_spaces = spaces.clone().to_owned();
        let position = empty_spaces.iter().position( |x: &char| *x == ' ' );

        match position {
            Some(index) => Some(index as int),
            None        => None
        }
    }

    fn minimax(&self, spaces: ~[char]) -> Option<int> {
        Some(-1)
    }

    pub fn clone(&self) -> AI {
        AI { strategy: self.strategy.clone() }
    }
}
