pub fn build() {
    println!("[build] llama.cpp/common");
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-deprecated-declarations")
        .include("/cpp") // adjust include path
        .include("../../external/llama.cpp/ggml/include")
        .include("../../external/llama.cpp/include")
        .include("../../external/llama.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-context.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-impl.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-model.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-graph.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-model-loader.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-mmap.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-arch.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-hparams.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-cparams.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-adapter.cpp") // adjust include path
        .file("../../external/llama.cpp/src/unicode.cpp") // adjust include path
        .file("../../external/llama.cpp/src/unicode-data.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-kv-cache.cpp") // adjust include path
        .file("../../external/llama.cpp/src/llama-vocab.cpp") // adjust include path
        .file("../../external/llama.cpp/common/common.cpp") // adjust include path
        .file("../../external/llama.cpp/ggml/src/ggml-backend-reg.cpp") // adjust include path
        //.file("../../external/llama.cpp/src/llama.cpp")
        .file("cpp/llama_shim.cpp")
        .compile("llrs_shim");
}
