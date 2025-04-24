use llrs_ffi::cuda::cuda__add;

#[test]
#[cfg(feature = "cuda")]
fn test_cuda_add() {
    unsafe {
        cuda__add(6, 9);
    }
}
