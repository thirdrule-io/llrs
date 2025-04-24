#include "llama.h"

// hack to get around i generate but not generate
// these are still mangled for some reason depsite the extern c in the bindgen generated file
// nm external/llama.cpp/build/src/libllama.a | grep llama_model_load
// 0000000000001d40 T llama_model_load_from_file
// 00000000000005d3 t llama_model_load_from_file.cold
// 0000000000001f40 T llama_model_load_from_splits
// 00000000000005f9 t llama_model_load_from_splits.cold
// 0000000000000e30 t _ZL31llama_model_load_from_file_implRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEERSt6vectorIS4_SaIS4_EE18llama_model_params
// 0000000000000120 t _ZL31llama_model_load_from_file_implRKNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEERSt6vectorIS4_SaIS4_EE18llama_model_params.cold
//                  U _ZN11llama_model10load_statsER18llama_model_loader
//                  U _ZN11llama_model10load_vocabER18llama_model_loader
//                  U _ZN11llama_model12load_hparamsER18llama_model_loader
//                  U _ZN11llama_model12load_tensorsER18llama_model_loader
//                  U _ZN11llama_model9load_archER18llama_model_loader
extern "C"
{
    // Shim for loading the model from file using the safe C interface
    llama_model *llama_model_load_from_file_c(const char *path)
    {

        fprintf(stderr, "[shim] llama_model_load_from_file_c() called\n");
        fprintf(stderr, "[shim] path = %s\n", path);
        llama_model_params params = llama_model_default_params(); // you may want to customize this
        fprintf(stderr, "[shim] got default params\n");

        llama_model *model = llama_load_model_from_file(path, params);
        if (model == nullptr)
        {
            fprintf(stderr, "[shim] llama_model_load returned NULL\n");
        }
        else
        {
            fprintf(stderr, "[shim] llama_model_load SUCCESS: %p\n", (void *)model);
        }
        return model; // <-- return the one you just loaded!
    }

    llama_context *llama_new_context_with_model_c(llama_model *model)
    {
        llama_context_params ctx_params = llama_context_default_params(); // you may want to customize this too
        return llama_new_context_with_model(model, ctx_params);
    }
}
