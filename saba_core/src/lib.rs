#! [no_std]

extern crate alloc;

pub mod browser;
pub mod url;
pub mod http;
pub mod error;
pub mod renderer;
pub mod utils;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
