pub mod b2u {
    use crate::tm::{Direction, Machine, MachineError, Symbol};

    #[derive(Copy, Clone, PartialEq)]
    enum State {
        Init,
        Sub,
        Shift,
        Add,
        Reset,
    }

    pub fn run(input: &str) -> Result<String, MachineError> {
        let mut tm = Machine::new(input, State::Init);
        loop {
            let state = tm.state();
            let symbol = tm.read();
            match state {
                State::Init => match symbol {
                    Symbol::Blank => tm.transition(State::Sub, Direction::Left, symbol),
                    _ => tm.transition(state, Direction::Right, symbol),
                },
                State::Sub => match symbol {
                    Symbol::Zero => tm.transition(state, Direction::Left, symbol),
                    Symbol::One => tm.transition(State::Shift, Direction::Right, Symbol::Zero),
                    _ => break Ok(tm.value().replace("0", "")),
                },
                State::Shift => match symbol {
                    Symbol::Zero => tm.transition(state, Direction::Right, Symbol::One),
                    Symbol::One => tm.transition(state, Direction::Left, Symbol::Zero),
                    _ => tm.transition(State::Add, Direction::Right, symbol),
                },
                State::Add => match symbol {
                    Symbol::One => tm.transition(state, Direction::Right, symbol),
                    Symbol::Blank => tm.transition(State::Reset, Direction::Left, Symbol::One),
                    _ => break Err(MachineError::HALT),
                },
                State::Reset => match symbol {
                    Symbol::One => tm.transition(state, Direction::Left, symbol),
                    Symbol::Blank => tm.transition(State::Sub, Direction::Left, symbol),
                    _ => break Err(MachineError::HALT),
                },
            }
        }
    }
}

pub mod u2b {
    use crate::tm::{Direction, Machine, MachineError, Symbol};

    #[derive(Copy, Clone, PartialEq)]
    enum State {
        Scan,
        Take,
        MoveL,
        Add,
        Carry,
        Reset,
    }

    pub fn run(input: &str) -> Result<String, MachineError> {
        let mut tm = Machine::new(input, State::Scan);
        loop {
            let state = tm.state();
            let symbol = tm.read();
            match state {
                State::Scan => match symbol {
                    Symbol::One => tm.transition(state, Direction::Right, symbol),
                    Symbol::Blank => tm.transition(State::Take, Direction::Left, symbol),
                    _ => break Err(MachineError::HALT),
                },
                State::Take => match symbol {
                    Symbol::One => tm.transition(State::MoveL, Direction::Left, Symbol::Blank),
                    Symbol::Blank => break Ok(tm.value()),
                    _ => break Err(MachineError::HALT),
                },
                State::MoveL => match symbol {
                    Symbol::One => tm.transition(state, Direction::Left, symbol),
                    Symbol::Blank => tm.transition(State::Add, Direction::Left, symbol),
                    _ => break Err(MachineError::HALT),
                },
                State::Add => match symbol {
                    Symbol::One => tm.transition(State::Carry, Direction::Left, Symbol::Zero),
                    _ => tm.transition(State::Reset, Direction::Right, Symbol::One),
                },
                State::Carry => match symbol {
                    Symbol::One => tm.transition(state, Direction::Left, Symbol::Zero),
                    _ => tm.transition(State::Reset, Direction::Right, Symbol::One),
                },
                State::Reset => match symbol {
                    Symbol::Blank => tm.transition(State::Scan, Direction::Right, symbol),
                    _ => tm.transition(state, Direction::Right, symbol),
                },
            }
        }
    }
}
