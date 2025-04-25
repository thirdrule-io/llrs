use std::env;

mod cuda {
    include!("build_cuda.rs");
}

// check for feature
fn has_feature(feature: &str) -> bool {
    std::env::vars().any(|(key, _)| {
        key == format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"))
    })
}

// main build
fn main() {
    cuda::build();

    let feature_cuda = has_feature("cuda");
    if feature_cuda {
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

    println!("cargo:rustc-link-arg=-Wl,--start-group");
    if feature_cuda {
        println!("cargo:rustc-link-lib=static=llrsggmlcuda");
    }
    println!("cargo:rustc-link-arg=-Wl,--end-group");
    // C++ runtime
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
