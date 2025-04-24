extern crate llrs_ffi;

use std::ffi::CString;
use std::ptr;

use llrs_ffi::ggml::{ggml_context, gguf_get_version, gguf_init_from_file, gguf_init_params};

#[test]
fn test_gguf_get_version() {
    // path to model
    let path =
        CString::new("../../../models/distilgpt2.Q4_K_M.gguf").expect("CString conversion failed");
    println!("[DEBUG test_gguf_get_version]");
    // Default/empty params

    // Create null pointers for the context
    let ggml_ctx: *mut ggml_context = ptr::null_mut();
    let mut ggml_ctx_ptr: *mut ggml_context = ggml_ctx;

    let params = gguf_init_params {
        no_alloc: false,
        ctx: &mut ggml_ctx_ptr,
    };

    unsafe {
        let ctx = gguf_init_from_file(path.as_ptr(), params);
        assert!(!ctx.is_null(), "Context should not be null");

        let version = gguf_get_version(ctx);
        println!("GGUF Version: {}", version);

        // TODO: wrap and call gguf_free(ctx) once you expose it via FFI
    }
}
