// // In src/llama.rs
pub mod llama_safe {

    pub use crate::llama_bindings::{
        llama_batch, llama_context, llama_context_params, llama_model, llama_model_params,
        llama_token,
    };
    pub use crate::llama_shim_bindings::llama_context as llamaShimContext;
    use std::ffi::{CStr, CString};

    unsafe extern "C" {
        pub fn llama_model_load(
            fname: *const ::std::os::raw::c_char,
            params: llama_model_params,
        ) -> *mut llama_model;

        pub fn llama_new_context_with_model(
            model: *mut llama_model,
            params: llama_context_params,
        ) -> *mut llama_context;

        pub fn llama_tokenize(
            ctx: *mut llama_context,
            text: *const i8,
            tokens: *mut i32,
            n_max_tokens: i32,
            add_bos: bool,
        ) -> i32;

        pub fn llama_token_to_piece(ctx: *mut llama_context, token: i32) -> *const i8;

        pub fn llama_batch_get_one(tokens: *mut llama_token, n_tokens: i32) -> llama_batch;

        pub fn llama_get_logits(ctx: *mut llama_context) -> *mut f32;
    }

    pub unsafe fn load_model(
        fname: *const ::std::os::raw::c_char,
        params: llama_model_params,
    ) -> *mut llama_model {
        unsafe { llama_model_load(fname, params) }
    }

    pub unsafe fn create_context(
        model: *mut llama_model,
        params: llama_context_params,
    ) -> *mut llama_context {
        unsafe { llama_new_context_with_model(model, params) }
    }

    pub unsafe fn tokenize(ctx: *mut llama_context, text: &str, add_bos: bool) -> Vec<llama_token> {
        let c_text = CString::new(text).expect("CString::new failed");
        let max_tokens = 512;
        let mut output_tokens = vec![0 as llama_token; max_tokens];

        println!(
            "[debug] input string = '{}', max_tokens = {}",
            text, max_tokens
        );

        let mut n_tokens = llama_tokenize(
            ctx,
            c_text.as_ptr(),
            output_tokens.as_mut_ptr(),
            max_tokens as i32,
            add_bos,
        );
        println!("[debug] llama_tokenize returned {}", n_tokens);

        if n_tokens < 0 {
            let needed = -n_tokens;
            output_tokens.resize(needed as usize, 0);
            n_tokens = llama_tokenize(
                ctx,
                c_text.as_ptr(),
                output_tokens.as_mut_ptr(),
                needed,
                add_bos,
            );
            //panic!("[error] llama_tokenize failed with code {}", n_tokens);
            assert_eq!(
                n_tokens, needed,
                "Retry did not match expected token count!"
            );
        }
        if (n_tokens as usize) > max_tokens {
            // panic!(
            //     "[error] Token count ({}) exceeds buffer capacity ({})!",
            //     n_tokens, max_tokens
            // );
        }

        output_tokens.truncate(n_tokens as usize);
        println!("[debug] tokenized output: {:?}", output_tokens);
        output_tokens
    }

    // pub unsafe fn tokenize(ctx: *mut llama_context, text: &str, add_bos: bool) -> Vec<llama_token> {
    //     let c_text = CString::new(text).expect("CString::new failed");

    //     let max_tokens = 512;
    //     let mut output_tokens = vec![0 as llama_token; max_tokens];

    //     let n_tokens = llama_tokenize(
    //         ctx,
    //         c_text.as_ptr(),
    //         output_tokens.as_mut_ptr(),
    //         max_tokens as i32,
    //         add_bos,
    //     );

    //     output_tokens.truncate(n_tokens as usize);
    //     output_tokens
    // }
    /// Convert a token ID to its string representation
    pub unsafe fn token_to_piece(ctx: *mut llama_context, token: i32) -> String {
        unsafe {
            let c_str = llama_token_to_piece(ctx, token);
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }

    /// Call llama_batch_get_one (typically used during decoding)
    pub unsafe fn batch_get_one(tokens: *mut llama_token, n_tokens: i32) -> llama_batch {
        unsafe { llama_batch_get_one(tokens, n_tokens) }
    }

    /// Get pointer to logits
    pub unsafe fn get_logits(ctx: *mut llama_context, vocab_size: usize) -> &'static [f32] {
        unsafe {
            let ptr = llama_get_logits(ctx);
            std::slice::from_raw_parts(ptr, vocab_size)
        }
    }
}
