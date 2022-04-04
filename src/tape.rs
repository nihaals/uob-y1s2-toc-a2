use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TapeValue<T: Clone> {
    Empty,
    Value(T),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TapeConstructor<T: Clone> {
    Value(TapeValue<T>),
    Head(TapeValue<T>),
}

impl<T: Clone> From<TapeConstructor<T>> for TapeValue<T> {
    fn from(tape_constructor: TapeConstructor<T>) -> Self {
        match tape_constructor {
            TapeConstructor::Value(tape_value) | TapeConstructor::Head(tape_value) => tape_value,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Tape<T: Clone> {
    head: usize,
    data: Vec<TapeValue<T>>,
    steps_ran: Option<Rc<RefCell<u64>>>,
}

impl<T: Clone> Tape<T> {
    pub fn new(initial_data: Vec<TapeConstructor<T>>, steps_ran: Option<Rc<RefCell<u64>>>) -> Self {
        let mut data = Vec::new();
        let mut head: Option<usize> = None;
        for (index, tape_constructor) in initial_data.into_iter().enumerate() {
            if let TapeConstructor::Head(_) = tape_constructor {
                assert!(head.is_none(), "Tape can only have one head");
                head = Some(index);
            }
            data.push(TapeValue::from(tape_constructor));
        }
        if let Some(head) = head {
            Self {
                head,
                data,
                steps_ran,
            }
        } else {
            panic!("Tape must have a head");
        }
    }

    fn increment_steps_ran(&self) {
        if let Some(steps_ran) = self.steps_ran.as_ref() {
            *steps_ran.borrow_mut() += 1;
        }
    }

    // pub fn steps_ran(&self) -> u64 {
    //     *self.steps_ran.as_ref().expect("Tape must have a steps_ran").borrow()
    // }

    pub fn is_at_head(&self) -> bool {
        self.head == 0
    }

    pub fn left(&mut self) {
        self.increment_steps_ran();

        if !self.is_at_head() {
            self.head -= 1;
        } else {
            self.data.insert(0, TapeValue::Empty);
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.head == self.data.len() - 1
    }

    pub fn right(&mut self) {
        self.increment_steps_ran();

        if self.is_at_end() {
            self.data.push(TapeValue::Empty);
        }
        self.head += 1;
    }

    // #[must_use]
    // pub fn read_ref(&self) -> &TapeValue<T> {
    //     &self.data[self.head]
    // }

    #[must_use]
    pub fn read(&self) -> TapeValue<T> {
        self.increment_steps_ran();

        self.data[self.head].clone()
    }

    pub fn write(&mut self, value: TapeValue<T>) {
        self.increment_steps_ran();

        self.data[self.head] = value;
    }

    #[must_use]
    pub fn data(&self) -> &[TapeValue<T>] {
        &self.data
    }

    #[must_use]
    pub fn head(&self) -> usize {
        self.head
    }

    #[must_use]
    pub fn as_constructor(&self) -> Vec<TapeConstructor<T>> {
        let mut tape_constructors = Vec::new();
        for (index, value) in self.data.iter().enumerate() {
            if index == self.head {
                tape_constructors.push(TapeConstructor::Head(value.clone()));
            } else {
                tape_constructors.push(TapeConstructor::Value(value.clone()));
            }
        }
        tape_constructors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let steps_ran = Rc::new(RefCell::new(0));

        let mut tape = Tape::new(
            vec![
                TapeConstructor::Value(TapeValue::Empty),
                TapeConstructor::Head(TapeValue::Value(0)),
                TapeConstructor::Value(TapeValue::Empty),
            ],
            Some(Rc::clone(&steps_ran)),
        );
        assert_eq!(tape.head, 1);
        assert_eq!(
            tape.data,
            vec![TapeValue::Empty, TapeValue::Value(0), TapeValue::Empty]
        );
        assert_eq!(tape.read(), TapeValue::Value(0));
        // First
        tape.left();
        assert_eq!(tape.read(), TapeValue::Empty);
        assert!(tape.is_at_head());
        // Out of bounds
        tape.left();
        assert_eq!(tape.read(), TapeValue::Empty);
        // First
        tape.right();
        assert_eq!(tape.read(), TapeValue::Empty);
        // Original head
        tape.right();
        assert_eq!(tape.read(), TapeValue::Value(0));
        // Third
        tape.right();
        assert_eq!(tape.read(), TapeValue::Empty);
        assert!(tape.is_at_end());
        // Out of bounds
        tape.right();
        assert_eq!(tape.read(), TapeValue::Empty);

        assert_eq!(
            tape.data,
            vec![
                TapeValue::Empty,
                TapeValue::Empty,
                TapeValue::Value(0),
                TapeValue::Empty,
                TapeValue::Empty
            ]
        );
        assert_eq!(tape.head, 4);

        assert_eq!(*steps_ran.borrow(), 13);
    }

    #[test]
    #[should_panic(expected = "Tape must have a head")]
    fn new_empty() {
        Tape::<()>::new(vec![], None);
    }

    #[test]
    #[should_panic(expected = "Tape must have a head")]
    fn new_no_head() {
        Tape::<()>::new(vec![TapeConstructor::Value(TapeValue::Empty)], None);
    }

    #[test]
    #[should_panic(expected = "Tape can only have one head")]
    fn new_multiple_heads() {
        Tape::<()>::new(
            vec![
                TapeConstructor::Value(TapeValue::Empty),
                TapeConstructor::Head(TapeValue::Empty),
                TapeConstructor::Head(TapeValue::Empty),
            ],
            None,
        );
    }
}
