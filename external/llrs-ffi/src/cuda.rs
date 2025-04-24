#[cfg(feature = "cuda")]
#[link(name = "llrs_cuda", kind = "static")]
unsafe extern "C" {
    pub fn cuda__add(a: i32, b: i32);
}
