use log::trace;
use std::{cell::RefCell, rc::Rc};

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

pub struct DestroyOutput {
    main_tape: Tape<MainValue>,
    aux_tape: Tape<AuxValue>,
}

impl DestroyOutput {
    pub fn new(main_tape: Tape<MainValue>, aux_tape: Tape<AuxValue>) -> Self {
        Self {
            main_tape,
            aux_tape,
        }
    }

    pub fn main_tape(&self) -> &Tape<MainValue> {
        &self.main_tape
    }

    pub fn aux_tape(&self) -> &Tape<AuxValue> {
        &self.aux_tape
    }
}

pub trait TuringMachine {
    fn new(main_tape: Tape<MainValue>, aux_tape: Tape<AuxValue>) -> Self;
    fn run(&mut self) -> bool;
    fn destroy(self) -> DestroyOutput;
}

pub fn choose() -> bool {
    rand::random()
}
