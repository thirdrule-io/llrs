#[cfg(feature = "cuda")]
#[link(name = "llrsggmlcuda", kind = "static")]
unsafe extern "C" {
    pub fn add(a: i32, b: i32);
}
