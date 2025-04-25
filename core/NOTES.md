let node = RuntimeNode::new("llama-core")
    .with_input(Stream::Prompt("Tell me a joke"))
    .with_backend("CUDA")
    .with_format("GGUF");

pipeline.add(node);
