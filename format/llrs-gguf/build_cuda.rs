use std::{env, path::PathBuf, process::Command};

pub fn build() {
    println!("[build] llama.cpp/ggml");

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cuda_src = manifest_dir.join("src/backend/cuda/kernels/add.cu");
    let cuda_obj = PathBuf::from(&out_dir).join("cuda_add.o");

    let status = Command::new("nvcc")
        .args([
            "-c",
            cuda_src.to_str().unwrap(),
            "-o",
            cuda_obj.to_str().unwrap(),
            "--compiler-options",
            "-fPIC",
        ])
        .status()
        .expect("Failed to compile CUDA");

    assert!(status.success(), "CUDA compilation failed");

    cc::Build::new()
        .cpp(true)
        //.include(manifest_dir.join("cpp/ggml"))
        //.file(manifest_dir.join("cpp/ggml/__ggml_hello.cpp"))
        .object(cuda_obj)
        .compile("llrsggmlcuda");

    // cc::Build::new()
    //     .cpp(true)
    //     .file("cpp/ggml/__ggml_hello.cpp")
    //     .include("cpp/ggml")
    //     .compile("llamars_ggml_src");

    //println!("cargo:rerun-if-changed=cpp/ggml/__ggml_hello.cpp");
    println!("cargo:rerun-if-changed=src/backend/cuda/kernels/add.cu");
}
