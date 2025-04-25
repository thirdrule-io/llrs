// https://medium.com/@phillipgimmi/what-is-gguf-and-ggml-e364834d241c

// struct gguf_context {
//     uint32_t version = GGUF_VERSION;

//     std::vector<struct gguf_kv> kv;
//     std::vector<struct gguf_tensor_info> info;

//     size_t alignment = GGUF_DEFAULT_ALIGNMENT;
//     size_t offset    = 0; // offset of `data` from beginning of file
//     size_t size      = 0; // size of `data` in bytes

//     void * data = nullptr;
// };

//struct gguf_con
