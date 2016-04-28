# rfloat
Rust crate that define some floating point types that are roundable (bypass the llvm optimization passes). The new floating point types will depend on the feature present on the user CPU (ex: if the SSE is present but not the SSE2, all the double float arithmetic will be executed on the fpu unit).
