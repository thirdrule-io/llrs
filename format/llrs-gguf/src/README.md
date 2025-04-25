src/
├── backend/         # CUDA / CPU / SIMD kernels
│   ├── cuda/
│   ├── cpu/
│   ├── simd/
├── ops/             # non-backend related operations
├── tensor/          # tensor definitions
├── engine/          # Planner, graph, buffer mgmt
├── io/              # GGUF reader/writer
└── lib.rs
