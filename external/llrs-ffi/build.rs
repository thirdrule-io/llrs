use std::env;

mod llrs_cuda {
    include!("build_cuda.rs");
}

mod llrs_shim {
    include!("build_shim.rs");
}

fn has_feature(feature: &str) -> bool {
    std::env::vars().any(|(key, _)| {
        key == format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"))
    })
}

pub fn generate_bindings(header: &str, out_file: &str, extra_args: &[&str]) {
    let mut common_args = vec![
        "-I../../external/llama.cpp/include",
        "-I../../external/llama.cpp/ggml/include",
        "-I../../external/llama.cpp/ggml/src",
        "-std=c++17",
        "-xc++",
    ];

    common_args.extend_from_slice(extra_args); // Add your extra per-call args

    let mut builder = bindgen::Builder::default().header(header);
    // .raw_line("#![allow(non_camel_case_types)]")
    // .raw_line("#![allow(non_snake_case)]")
    // .raw_line("#![allow(non_upper_case_globals)]")
    // .raw_line("#![allow(dead_code)]");

    for arg in common_args {
        builder = builder.clang_arg(arg);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("buildin");

    generate_bindings("headers/wrapper_ggml.h", "bindings/ggml_bindings.rs", &[]);
    generate_bindings("headers/wrapper_gguf.h", "bindings/gguf_bindings.rs", &[]);
    generate_bindings("headers/wrapper_llama.h", "bindings/llama_bindings.rs", &[]);
    generate_bindings(
        "headers/wrapper_common.h",
        "bindings/common_bindings.rs",
        &[
            "-I../../external/llama.cpp/common",
            "-I../../external/llama.cpp",
            "-I/usr/include/c++/11",
            "-I/usr/include/x86_64-linux-gnu/c++/11",
        ],
    );

    llrs_shim::build();

    let has_cuda = has_feature("cuda");
    match has_feature("cuda") {
        true => {
            llrs_cuda::build();

            const BIN_DIR: &str = "/usr/bin";
            let mut path = env::var("PATH").unwrap_or_default();
            if !path.split(':').any(|p| p == BIN_DIR) {
                path = format!("{BIN_DIR}:{path}");
                unsafe {
                    env::set_var("PATH", &path); // PATH=/usr/bin:…
                }
            }
            unsafe {
                env::set_var("COMPILER_PATH", BIN_DIR); // lets g++ find `as`
            }
            let cuda_home = env::var("CUDA_HOME").unwrap_or("/usr/local/cuda".into());

            println!("cargo:rustc-link-lib=dylib=cudart");
            println!("cargo:rustc-link-search=native={cuda_home}/lib64");
            let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
            println!("cargo:rustc-link-search=native={}", out_dir);
        }
        _ => println!("[build] Skipping CUDA build due to CI env or SKIP_CUDA flag."),
    }

    println!("cargo:rustc-link-arg=-Wl,--start-group");
    let full_path = std::fs::canonicalize("../../external/llama.cpp/build/ggml/src")
        .expect("Could not resolve path to libggml-base.a");
    println!("cargo:rustc-link-search=native={}", full_path.display());
    println!("cargo:rustc-link-lib=static=ggml-base");
    //println!("cargo:rustc-link-lib=static=llamars_ffi_src");
    // println!("cargo:rustc-link-lib=static=ggml-base");
    if has_cuda {
        //println!("cargo:rustc-link-lib=static=llamars_ggml_src");
        println!("cargo:rustc-link-lib=static=llrs_cuda");
    }
    println!("cargo:rustc-link-lib=static=llrs_shim");
    println!("cargo:rustc-link-arg=-Wl,--end-group");
    // C++ runtime
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
