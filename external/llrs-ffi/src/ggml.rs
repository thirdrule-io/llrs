include!("../bindings/gguf_bindings.rs");

pub fn gguf_version() -> String {
    let version = unsafe { gguf_get_version(std::ptr::null()) };
    version.to_string()
}
