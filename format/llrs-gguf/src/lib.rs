// mod gguf;
// mod tensor;
// mod types;

// pub fn add(left: u64, right: u64) -> u64 {
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
pub mod backend {
    pub mod cuda {
        pub mod ops {
            pub mod add; // or `pub use self::add::*;` if you want to re-export symbols
        }
    }
}
