use log::trace;

use crate::{
    machine::{choose, AuxValue, DestroyOutput, MainValue, TuringMachine},
    tape::{Tape, TapeConstructor, TapeValue},
};

pub struct M2 {
    main_tape: Tape<MainValue>,
    aux_tape: Tape<AuxValue>,
}

impl TuringMachine for M2 {
    fn new(main_tape: Tape<MainValue>, aux_tape: Tape<AuxValue>) -> Self {
        {
            let mut main_tape = main_tape.clone();
            // All cells to the left of the head and the head itself must be empty
            for cell in main_tape.as_constructor().iter() {
                match cell {
                    TapeConstructor::Head(TapeValue::Empty) => {
                        break;
                    }
                    TapeConstructor::Value(TapeValue::Empty) => {
                        continue;
                    }
                    TapeConstructor::Value(TapeValue::Value(_)) => {
                        panic!("M2 main tape must only have empty cells on the left of the head");
                    }
                    TapeConstructor::Head(TapeValue::Value(_)) => {
                        panic!("M2 head must be empty")
                    }
                };
            }

            // Head must be followed by a non-empty word
            let mut found_a_b = false;
            loop {
                main_tape.right();
                match main_tape.read() {
                    TapeValue::Value(MainValue::A | MainValue::B) => {
                        found_a_b = true;
                    }
                    TapeValue::Value(MainValue::Hash) => {
                        break;
                    }
                    TapeValue::Empty => {
                        panic!("M2 main tape word must be followed by a hash");
                    }
                };
            }
            assert!(found_a_b);
        }

        {
            // Aux tape must be blank
            let aux_tape = aux_tape.clone();
            for cell in aux_tape.as_constructor().iter() {
                match cell {
                    TapeConstructor::Head(TapeValue::Value(_))
                    | TapeConstructor::Value(TapeValue::Value(_)) => {
                        panic!("M2 aux tape must be blank");
                    }
                    _ => {}
                };
            }
        }

        Self {
            main_tape,
            aux_tape,
        }
    }

    fn run(&mut self) -> bool {
        self.zero()
    }

    fn destroy(self) -> DestroyOutput {
        DestroyOutput::new(self.main_tape, self.aux_tape)
    }
}

// Run
impl M2 {
    fn zero(&mut self) -> bool {
        // 0
        trace!("M2: 0");
        self.main_tape.right();
        // 1
        trace!("M2: 1");
        let read = self.main_tape.read();
        match read {
            TapeValue::Empty => {
                unreachable!()
            }
            TapeValue::Value(MainValue::A | MainValue::B) => {
                // 2
                trace!("M2: 2");
                self.aux_tape.right();
                // 3
                trace!("M2: 3");
                let pick = choose();
                if pick {
                    // 4
                    trace!("M2: 4");
                    self.aux_tape.write(TapeValue::Value(AuxValue::A));
                    // 0
                    self.zero()
                } else {
                    // 5
                    trace!("M2: 5");
                    self.aux_tape.write(TapeValue::Value(AuxValue::B));
                    // 0
                    self.zero()
                }
            }
            TapeValue::Value(MainValue::Hash) => {
                // 6
                self.six()
            }
        }
    }

    fn six(&mut self) -> bool {
        // 6
        trace!("M2: 6");
        self.aux_tape.left();
        // 7
        trace!("M2: 7");
        let read = self.aux_tape.read();
        match read {
            TapeValue::Value(AuxValue::A | AuxValue::B) => {
                // 6
                self.six()
            }
            TapeValue::Empty => {
                // 8
                trace!("M2: 8");
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let main_tape = Tape::new(
            vec![
                TapeConstructor::Head(TapeValue::Empty),
                TapeConstructor::Value(TapeValue::Value(MainValue::A)),
                TapeConstructor::Value(TapeValue::Value(MainValue::B)),
                TapeConstructor::Value(TapeValue::Value(MainValue::A)),
                TapeConstructor::Value(TapeValue::Value(MainValue::B)),
                TapeConstructor::Value(TapeValue::Value(MainValue::Hash)),
            ],
            None,
        );
        let aux_tape = Tape::new(vec![TapeConstructor::Head(TapeValue::Empty)], None);
        let mut m2 = M2::new(main_tape.clone(), aux_tape);
        m2.run();
        let output = m2.destroy();

        assert_eq!(output.main_tape().data(), main_tape.data());
        assert_eq!(output.main_tape().head(), 5);
        assert_eq!(output.main_tape().read(), TapeValue::Value(MainValue::Hash));

        assert_eq!(output.aux_tape().read(), TapeValue::Empty);
        assert_eq!(output.aux_tape().head(), 0);
        for index in 1..5 {
            match output.aux_tape().data()[index] {
                TapeValue::Value(AuxValue::A | AuxValue::B) => {}
                _ => panic!(),
            };
        }
        assert_eq!(
            output.aux_tape().data().get(5).unwrap_or(&TapeValue::Empty),
            &TapeValue::Empty
        );
    }
}
