cd llama.cpp
mkdir -p build && cd build

cmake .. \
  -DLLAMA_BUILD_EXAMPLES=ON \
  -DLLAMA_BUILD_TESTS=OFF \
  -DLLAMA_CUBLAS=OFF \
  -DLLAMA_METAL=OFF \
  -DGGML_CUDA=ON \
  -DGGML_STATIC=ON \
  -DBUILD_SHARED_LIBS=OFF  # Force static linking for everything

make -j4
