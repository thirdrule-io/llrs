llama-rs/
├── core/       # core inference, runtime, kv-cache, kernels
├── docs/       # documentation
├── format/     # gguf, ggml, quant loaders
├── tools/      # CLI, REPL, daemons
├── research/   # experimental stuff, research-grade algorithms
└── extern/     # vendored or patched deps (e.g., llama-cpp-sys)


core/
├── llama-core          → Model structs, token types, basic utilities
├── llama-runtime       → Execution engine, graph, context mgmt
├── llama-kernels       → SIMD/CUDA-backed ops, math kernels


format/
├── llama-gguf          → GGUF file parser/writer
├── llama-ggml          → Raw tensor format, alignment, quantization-aware
├── llama-loader        → High-level model loader from disk into runtime


tools/
├── llama-cli           → CLI for inference
├── llama-chat          → REPL for conversation
├── llama-daemon        → Long-running inference server
├── llama-bench         → Performance benchmarking


research/
├── llama-algos         → Experimental attention/FFN replacements, etc.
├── llama-experiments   → Custom models, scaffolds, metrics
├── llama-research-utils → Tokenizers, profilers, viz tools


extern/
├── llama-cpp-sys       → FFI + build.rs for llama.cpp
├── ggml-patched        → Vendored GGML (with patches + SIMD tweaks)
