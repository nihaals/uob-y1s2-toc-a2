#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_enum)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::float_cmp)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::integer_division)]
#![warn(clippy::let_unit_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::many_single_char_names)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::negative_feature_names)]
#![warn(non_ascii_idents)]
#![warn(clippy::option_option)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::single_match_else)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::zero_sized_map_values)]

mod m1;
mod m2;
mod m3;
mod machine;
mod tape;

use machine::{AuxValue, MainValue, TuringMachine};
use tape::{TapeConstructor, TapeValue};

use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[clap(version, author, about, long_about = None)]
// struct Cli {
//     #[clap(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Run with specific values of n and m
//     Run {
//         /// n
//         #[clap(short)]
//         n: u64,

//         /// m
//         #[clap(short)]
//         m: u64,
//     },
// }

// fn n_m_steps(n: u64, m: u64) -> u64 {
//     if n <= m {
//         n * 6 + 7
//     } else if n > m {
//         n * 2 + m * 4 + 7
//     } else {
//         unreachable!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn main() {
//         for n in 1..101 {
//             for m in 1..101 {
//                 let mut main_tape = vec![TapeConstructor::Head(TapeValue::Value(MainValue::Hash))];
//                 for _ in 0..n {
//                     main_tape.push(TapeConstructor::Value(TapeValue::Value(MainValue::A)));
//                 }
//                 main_tape.push(TapeConstructor::Value(TapeValue::Value(MainValue::Hash)));

//                 let mut aux_tape = vec![TapeConstructor::Head(TapeValue::Empty)];
//                 for _ in 0..m {
//                     aux_tape.push(TapeConstructor::Value(TapeValue::Value(AuxValue::A)));
//                 }
//                 aux_tape.push(TapeConstructor::Value(TapeValue::Empty));

//                 let mut machine = TuringMachine::new(main_tape, aux_tape);

//                 let _ = machine.run();
//                 let steps = n_m_steps(n, m);
//                 assert_eq!(
//                     machine.steps_ran(),
//                     steps,
//                     "n={}, m={}, Actual steps: {}, Predicted steps: {}",
//                     n,
//                     m,
//                     machine.steps_ran(),
//                     steps
//                 );
//             }
//         }
//     }
// }

fn main() {
    // let mut machine = TuringMachine::new(
    //     vec![
    //         TapeConstructor::Head(TapeValue::Value(MainValue::Hash)),
    //         TapeConstructor::Value(TapeValue::Value(MainValue::A)),
    //         TapeConstructor::Value(TapeValue::Value(MainValue::B)),
    //         TapeConstructor::Value(TapeValue::Value(MainValue::B)),
    //         TapeConstructor::Value(TapeValue::Value(MainValue::A)),
    //         TapeConstructor::Value(TapeValue::Value(MainValue::Hash)),
    //     ],
    //     vec![
    //         TapeConstructor::Head(TapeValue::Empty),
    //         TapeConstructor::Value(TapeValue::Value(AuxValue::A)),
    //         TapeConstructor::Value(TapeValue::Value(AuxValue::B)),
    //         TapeConstructor::Value(TapeValue::Empty),
    //     ],
    // );

    env_logger::init();

    // let cli = Cli::parse();

    // match cli.command {
    //     Commands::Run { n, m } => {
    //         let mut main_tape = vec![TapeConstructor::Head(TapeValue::Value(MainValue::Hash))];
    //         for _ in 0..n {
    //             main_tape.push(TapeConstructor::Value(TapeValue::Value(MainValue::A)));
    //         }
    //         main_tape.push(TapeConstructor::Value(TapeValue::Value(MainValue::Hash)));

    //         let mut aux_tape = vec![TapeConstructor::Head(TapeValue::Empty)];
    //         for _ in 0..m {
    //             aux_tape.push(TapeConstructor::Value(TapeValue::Value(AuxValue::A)));
    //         }
    //         aux_tape.push(TapeConstructor::Value(TapeValue::Empty));

    //         let mut machine = TuringMachine::new(main_tape, aux_tape);
    //         println!(
    //             "Result: {}\nn={n}\nm={m}\nSteps: {}\nn_m_steps: {}",
    //             machine.run(),
    //             machine.steps_ran(),
    //             n_m_steps(n, m),
    //         );
    //     }
    // }
}
