#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

    struct llama_model;
    struct llama_context;

    // C-friendly wrapper for loading a model from file
    struct llama_model *llama_model_load_from_file_c(const char *path);

    // C-friendly wrapper for creating a context
    struct llama_context *llama_new_context_with_model_c(struct llama_model *model);

    int llama_tokenize(
        struct llama_context *ctx,
        const char *text,
        llama_token *tokens,
        int n_max_tokens,
        bool add_bos);

#ifdef __cplusplus
}
#endif
