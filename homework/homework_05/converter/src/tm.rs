#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub enum MachineError {
    HALT,
}

#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    Blank,
    Zero,
    One,
}

pub struct Machine<T> {
    head: usize,
    state: T,
    tape: Vec<Symbol>,
}

impl<T> Machine<T>
where
    T: Copy,
{
    pub fn new(input: &str, state: T) -> Machine<T> {
        Machine {
            head: 0,
            state,
            tape: make_tape(input),
        }
    }

    pub fn read(&self) -> Symbol {
        self.tape[self.head]
    }

    pub fn state(&self) -> T {
        self.state
    }

    pub fn transition(&mut self, state: T, direction: Direction, write: Symbol) {
        self.state = state;
        self.tape[self.head] = write;
        self.head = match direction {
            Direction::Left => {
                if self.head == 0 {
                    self.tape.insert(0, Symbol::Blank);
                    0
                } else {
                    self.head - 1
                }
            }
            Direction::Right => {
                if self.head == self.tape.len() - 1 {
                    self.tape.push(Symbol::Blank);
                }
                self.head + 1
            }
        }
    }

    pub fn value(&self) -> String {
        self.tape
            .iter()
            .filter_map(|symbol| match symbol {
                Symbol::Zero => Some('0'),
                Symbol::One => Some('1'),
                _ => None,
            })
            .collect()
    }
}

fn make_tape(input: &str) -> Vec<Symbol> {
    input
        .chars()
        .map(|c| match c {
            '1' => Symbol::One,
            '0' => Symbol::Zero,
            _ => {
                panic!("Input must only contain 1s and 0s");
            }
        })
        .collect()
}
