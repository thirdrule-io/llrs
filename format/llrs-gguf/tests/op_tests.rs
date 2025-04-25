extern crate llrs_gguf;

pub use llrs_gguf::backend::cuda::ops::add::add;

#[test]
fn test_cuda_op_add() {
    unsafe {
        add(12, 34);
    }
}
