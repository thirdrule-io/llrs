// struct ggml_tensor {
//     enum ggml_type type;

//     struct ggml_backend_buffer * buffer;

//     int64_t ne[GGML_MAX_DIMS]; // number of elements
//     size_t  nb[GGML_MAX_DIMS]; // stride in bytes:
//                                // nb[0] = ggml_type_size(type)
//                                // nb[1] = nb[0]   * (ne[0] / ggml_blck_size(type)) + padding
//                                // nb[i] = nb[i-1] * ne[i-1]

//     // compute data
//     enum ggml_op op;

//     // op params - allocated as int32_t for alignment
//     int32_t op_params[GGML_MAX_OP_PARAMS / sizeof(int32_t)];

//     int32_t flags;

//     struct ggml_tensor * src[GGML_MAX_SRC];

//     // source tensor and offset for views
//     struct ggml_tensor * view_src;
//     size_t               view_offs;

//     void * data;

//     char name[GGML_MAX_NAME];

//     void * extra; // extra things e.g. for ggml-cuda.cu

//     char padding[8];
// };

// tensor.rs
pub struct Tensor {
    pub data: *mut u8,               // raw ptr or smarter later
    pub shape: Vec<usize>,           // dims
    pub dtype: DType,                // quantized or f32
    pub strides: Option<Vec<usize>>, // calculated if needed
}
