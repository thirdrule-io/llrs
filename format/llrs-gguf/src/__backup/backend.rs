#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Backend {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ggml_backend_blas_context {}

// struct ggml_backend_buffer_type {
//     struct ggml_backend_buffer_type_i  iface;
//     ggml_backend_dev_t device;
//     void * context;
// };

// struct ggml_backend_buffer_type_i {
//     const char *          (*get_name)      (ggml_backend_buffer_type_t buft);
//     // allocate a buffer of this type
//     ggml_backend_buffer_t (*alloc_buffer)  (ggml_backend_buffer_type_t buft, size_t size);
//     // tensor alignment
//     size_t                (*get_alignment) (ggml_backend_buffer_type_t buft);
//     // (optional) max buffer size that can be allocated (defaults to SIZE_MAX)
//     size_t                (*get_max_size)  (ggml_backend_buffer_type_t buft);
//     // (optional) data size needed to allocate the tensor, including padding (defaults to ggml_nbytes)
//     size_t                (*get_alloc_size)(ggml_backend_buffer_type_t buft, const struct ggml_tensor * tensor);
//     // (optional) check if tensor data is in host memory and uses standard ggml tensor layout (defaults to false)
//     bool                  (*is_host)       (ggml_backend_buffer_type_t buft);
// };
pub trait BackendBUfferType {
    fn get_name(&self) -> &'static str;
}

// ggml_backend_buffer
// ggml_backend_buffer_deleter
// ggml_backend_buffer_i
// ggml_backend_buffer_type
// ggml_backend_buffer_type_i
// ggml_backend_cann_buffer_context
// ggml_backend_cann_buffer_type_context
// ggml_backend_cann_context
// ggml_backend_cann_device_context
// ggml_backend_cann_reg_context
// ggml_backend_cpu_context
// ggml_backend_cpu_device_context
// ggml_backend_cuda_buffer_context
// ggml_backend_cuda_buffer_type_context
// ggml_backend_cuda_context
// ggml_backend_cuda_device_context
// ggml_backend_cuda_reg_context
// ggml_backend_cuda_split_buffer_context
// ggml_backend_cuda_split_buffer_type_context
// ggml_backend_deleter
// ggml_backend_dev_caps
// ggml_backend_device
// ggml_backend_device_i
// ggml_backend_dev_props
// ggml_backend_event
// ggml_backend_event_deleter
// ggml_backend_feature
// ggml_backend_graph_copy
// //ggml_backend_i
// ggml_backend_kompute_buffer_type_context
// ggml_backend_kompute_device_context
// ggml_backend_metal_buffer
// ggml_backend_metal_buffer_context
// ggml_backend_metal_context
// ggml_backend_metal_device_context
// ggml_backend_multi_buffer_context
// ggml_backend_opencl_buffer_context
// ggml_backend_opencl_context
// ggml_backend_opencl_device_context
// ggml_backend_plan_cpu
// ggml_backend_reg
// ggml_backend_reg_entry
// ggml_backend_reg_i
// ggml_backend_registry
// ggml_backend_rpc_buffer_context
// ggml_backend_rpc_buffer_type_context
// ggml_backend_rpc_context
// ggml_backend_rpc_device_context
// ggml_backend_sched
// ggml_backend_sched_deleter
// ggml_backend_sched_split
// ggml_backend_sycl_buffer_context
// ggml_backend_sycl_buffer_type_context
// ggml_backend_sycl_context
// ggml_backend_sycl_device_context
// ggml_backend_sycl_reg_context
// ggml_backend_sycl_split_buffer_context
// ggml_backend_sycl_split_buffer_type_context
// ggml_backend_vk_buffer_context
// ggml_backend_vk_buffer_type_context
// ggml_backend_vk_context
// ggml_backend_vk_device_context
