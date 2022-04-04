use std::{cell::RefCell, rc::Rc};
use log::trace;

use crate::tape::{Tape, TapeConstructor, TapeValue};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MainValue {
    A,
    B,
    Hash,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AuxValue {
    A,
    B,
}

pub struct TuringMachine {
    main: Tape<MainValue>,
    aux: Tape<AuxValue>,
    steps_ran: Rc<RefCell<u64>>,
}

impl TuringMachine {
    pub fn new(main: Vec<TapeConstructor<MainValue>>, aux: Vec<TapeConstructor<AuxValue>>) -> Self {
        {
            let mut main_tape = Tape::new(main.clone(), None);
            assert_eq!(main_tape.read(), TapeValue::Value(MainValue::Hash));
            let mut found_a_b = false;
            loop {
                main_tape.right();
                match main_tape.read() {
                    TapeValue::Value(MainValue::A | MainValue::B) => {
                        found_a_b = true;
                    }
                    TapeValue::Value(MainValue::Hash) | TapeValue::Empty => {
                        break;
                    }
                };
            }
            assert!(found_a_b);
        }

        {
            let mut aux_tape = Tape::new(aux.clone(), None);

            // All cells on the left of the head and the head itself must be empty
            for cell in aux.iter() {
                match cell {
                    TapeConstructor::Head(TapeValue::Empty) => {
                        break;
                    }
                    TapeConstructor::Value(TapeValue::Empty) => {
                        continue;
                    }
                    TapeConstructor::Value(TapeValue::Value(_)) => {
                        panic!("Aux tape must only have empty cells on the left of the head");
                    }
                    TapeConstructor::Head(TapeValue::Value(_)) => {
                        panic!("Head must be empty")
                    }
                };
            }

            let mut found_a_b = false;
            loop {
                aux_tape.right();
                match aux_tape.read() {
                    TapeValue::Value(AuxValue::A | AuxValue::B) => {
                        found_a_b = true;
                    }
                    TapeValue::Empty => {
                        break;
                    }
                };
            }
            assert!(found_a_b);

            // All cells on the right of the word must be empty
            while !aux_tape.is_at_end() {
                aux_tape.right();
                assert_eq!(aux_tape.read(), TapeValue::Empty);
            }
        }

        let steps_ran = Rc::new(RefCell::new(0));
        Self {
            main: Tape::new(main, Some(Rc::clone(&steps_ran))),
            aux: Tape::new(aux, Some(Rc::clone(&steps_ran))),
            steps_ran,
        }
    }

    pub fn steps_ran(&self) -> u64 {
        *self.steps_ran.borrow()
        // self.main.steps_ran()
    }

    pub fn increment_steps_ran(&self) {
        *self.steps_ran.borrow_mut() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        TuringMachine::new(
            vec![
                TapeConstructor::Head(TapeValue::Value(MainValue::Hash)),
                TapeConstructor::Value(TapeValue::Value(MainValue::A)),
                TapeConstructor::Value(TapeValue::Value(MainValue::B)),
                TapeConstructor::Value(TapeValue::Value(MainValue::B)),
                TapeConstructor::Value(TapeValue::Value(MainValue::A)),
                TapeConstructor::Value(TapeValue::Value(MainValue::Hash)),
            ],
            vec![
                TapeConstructor::Head(TapeValue::Empty),
                TapeConstructor::Value(TapeValue::Value(AuxValue::A)),
                TapeConstructor::Value(TapeValue::Value(AuxValue::B)),
                TapeConstructor::Value(TapeValue::Empty),
            ],
        );
    }
}

// Run
impl TuringMachine {
    /// 0
    fn zero(&mut self) -> bool {
        // 0
        trace!("0");
        self.aux.right();
        // 1
        trace!("1");
        let read = self.aux.read();
        match read {
            TapeValue::Value(AuxValue::A) => {
                // 2
                self.two()
            }
            TapeValue::Value(AuxValue::B) => {
                // 6
                self.six()
            }
            TapeValue::Empty => {
                // 4
                self.four()
            }
        }
    }

    /// 2
    fn two(&mut self) -> bool {
        // 2
        trace!("2");
        self.main.right();
        // 3
        trace!("3");
        let read = self.main.read();
        match read {
            TapeValue::Value(MainValue::A) => {
                // 0
                self.zero()
            }
            TapeValue::Value(MainValue::B) => {
                // 2
                self.two()
            }
            TapeValue::Empty | TapeValue::Value(MainValue::Hash) => {
                // 8
                self.eight()
            }
        }
    }

    /// 4
    fn four(&mut self) -> bool {
        // 4
        trace!("4");
        self.aux.left();
        // 5
        trace!("5");
        let read = self.aux.read();
        match read {
            TapeValue::Value(AuxValue::A | AuxValue::B) => {
                // 4
                self.four()
            }
            TapeValue::Empty => {
                // 11
                self.eleven()
            }
        }
    }

    /// 6
    fn six(&mut self) -> bool {
        // 6
        trace!("6");
        self.main.right();
        // 7
        trace!("7");
        let read = self.main.read();
        match read {
            TapeValue::Value(MainValue::A) => {
                // 6
                self.six()
            }
            TapeValue::Value(MainValue::B) => {
                // 0
                self.zero()
            }
            TapeValue::Empty | TapeValue::Value(MainValue::Hash) => {
                // 8
                self.eight()
            }
        }
    }

    /// 8
    fn eight(&mut self) -> bool {
        // 8
        trace!("8");
        self.aux.left();
        // 9
        trace!("9");
        let read = self.aux.read();
        match read {
            TapeValue::Value(AuxValue::A | AuxValue::B) => {
                // 8
                self.eight()
            }
            TapeValue::Empty => {
                // 10
                trace!("10");
                self.increment_steps_ran();
                false
            }
        }
    }

    /// 11
    fn eleven(&mut self) -> bool {
        // 11
        trace!("11");
        self.main.right();
        // 12
        trace!("12");
        let read = self.main.read();
        match read {
            TapeValue::Value(MainValue::A | MainValue::B) => {
                // 11
                self.eleven()
            }
            TapeValue::Empty | TapeValue::Value(MainValue::Hash) => {
                // 13
                trace!("13");
                self.increment_steps_ran();
                true
            }
        }
    }

    #[must_use]
    pub fn run(&mut self) -> bool {
        assert_eq!(self.steps_ran(), 0, "Machine cannot be ran more than once");

        // 0
        self.zero()
    }
}
