use log::trace;

use crate::{
    m1::M1,
    m2::M2,
    machine::{choose, AuxValue, DestroyOutput, MainValue, TuringMachine},
    tape::{Tape, TapeConstructor, TapeValue},
};

pub struct M3 {
    main_tape: Tape<MainValue>,
    aux_tape: Tape<AuxValue>,
    substring: Option<String>,
}

impl TuringMachine for M3 {
    fn new(main_tape: Tape<MainValue>, aux_tape: Tape<AuxValue>) -> Self {
        {
            // abab#abab#abab
            let mut main_tape = main_tape.clone();
            // All cells to the left of the head and the head itself must be empty
            for cell in main_tape.as_constructor().iter() {
                match cell {
                    TapeConstructor::Value(TapeValue::Empty) => {
                        continue;
                    }
                    TapeConstructor::Value(TapeValue::Value(_)) => {
                        panic!("M3 main tape must have only empty cells on the left of the head");
                    }
                    TapeConstructor::Head(TapeValue::Empty) => {
                        break;
                    }
                    TapeConstructor::Head(_) => {
                        panic!("M3 main tape head must be empty");
                    }
                };
            }

            let mut found_empty = false;
            while !found_empty {
                let mut found_a_b = false;
                loop {
                    main_tape.right();
                    match main_tape.read() {
                        TapeValue::Value(MainValue::A | MainValue::B) => {
                            found_a_b = true;
                            continue;
                        }
                        TapeValue::Value(MainValue::Hash) => {
                            break;
                        }
                        TapeValue::Empty => {
                            found_empty = true;
                            break;
                        }
                    }
                }
                assert!(found_a_b);
            }

            // After first empty after words, must only be empty cells
            while !main_tape.is_at_end() {
                main_tape.right();
                assert!(main_tape.read() == TapeValue::Empty)
            }
        }

        {
            // Aux tape must be blank
            let aux_tape = aux_tape.clone();
            for cell in aux_tape.as_constructor().iter() {
                match cell {
                    TapeConstructor::Head(TapeValue::Value(_))
                    | TapeConstructor::Value(TapeValue::Value(_)) => {
                        panic!("M3 aux tape must be blank");
                    }
                    _ => {}
                };
            }
        }

        Self {
            main_tape,
            aux_tape,
            substring: None,
        }
    }

    #[must_use]
    fn run(&mut self) -> bool {
        self.zero()
    }

    fn destroy(self) -> DestroyOutput {
        DestroyOutput::new(self.main_tape, self.aux_tape)
    }
}

impl M3 {
    pub fn substring(&self) -> &str {
        self.substring.as_deref().unwrap()
    }
}

// Run
impl M3 {
    fn zero(&mut self) -> bool {
        // 0
        trace!("M3: 0");

        let mut m2 = M2::new(self.main_tape.clone(), self.aux_tape.clone());
        m2.run();
        let destroy = m2.destroy();
        self.main_tape = destroy.main_tape().clone();
        self.aux_tape = destroy.aux_tape().clone();

        self.substring = {
            let mut aux_tape = destroy.aux_tape().clone();
            let mut substring = String::new();
            loop {
                aux_tape.right();
                match aux_tape.read() {
                    TapeValue::Value(AuxValue::A) => {
                        substring.push('a');
                    }
                    TapeValue::Value(AuxValue::B) => {
                        substring.push('b');
                    }
                    TapeValue::Empty => {
                        break;
                    }
                };
            }
            assert!(!substring.is_empty());
            Some(substring)
        };

        // 1
        self.one()
    }

    fn one(&mut self) -> bool {
        // 1
        trace!("M3: 1");

        let mut m1 = M1::new(self.main_tape.clone(), self.aux_tape.clone());
        let output = m1.run();
        let destroy = m1.destroy();
        self.main_tape = destroy.main_tape().clone();
        self.aux_tape = destroy.aux_tape().clone();

        if output {
            // 3
            trace!("M3: 3");
            let read = self.main_tape.read();
            match read {
                TapeValue::Empty => {
                    // 4
                    trace!("M3: 4");
                    true
                }
                TapeValue::Value(MainValue::Hash) => {
                    // 1
                    self.one()
                }
                TapeValue::Value(_) => {
                    unreachable!()
                }
            }
        } else {
            // 2
            trace!("M3: 2");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_m3_main_tape(s: &str) -> Tape<MainValue> {
        let mut data = vec![TapeConstructor::Head(TapeValue::Empty)];
        for char in s.chars() {
            match char {
                'a' => data.push(TapeConstructor::Value(TapeValue::Value(MainValue::A))),
                'b' => data.push(TapeConstructor::Value(TapeValue::Value(MainValue::B))),
                '#' => data.push(TapeConstructor::Value(TapeValue::Value(MainValue::Hash))),
                _ => panic!("Invalid character in M3 main tape"),
            }
        }
        Tape::new(data, None)
    }

    fn is_present(s: &str, substring: &str) -> bool {
        let mut current_substring_char = substring.chars().peekable();
        for char in s.chars() {
            let current_goal = match current_substring_char.peek() {
                Some(c) => c,
                None => return true,
            };
            if &char == current_goal {
                current_substring_char.next();
            }
        }
        return current_substring_char.peek().is_none();
    }

    fn m3_own(s: &str, substring: &str) -> bool {
        let mut words = s.split('#');
        if words.next().unwrap().len() != substring.len() {
            return false;
        }
        for word in words {
            if !is_present(word, substring) {
                return false;
            }
        }
        true
    }

    #[test]
    fn run() {
        let s = "aba#aba#aba";
        let main_tape = str_to_m3_main_tape(s);
        let aux_tape = Tape::new(vec![TapeConstructor::Head(TapeValue::Empty)], None);
        let mut m3 = M3::new(main_tape, aux_tape);
        let output = m3.run();
        let substring = m3.substring();
        assert_eq!(m3_own(s, substring), output);
    }

    #[test]
    fn test_is_present() {
        assert!(is_present("abba", "aa"));
        assert!(is_present("abbba", "aba"));
        assert!(is_present("abbba", "abbba"));
        assert!(!is_present("abba", "aab"));
    }

    #[test]
    fn test_m3_own() {
        assert!(m3_own("abba#abba#abba", "abba"));
        assert!(m3_own("abba#abbba#abba", "abba"));
        assert!(!m3_own("abba#aba", "abba"));
        assert!(!m3_own("abba#abbba#abba", "abaa"));
    }
}
