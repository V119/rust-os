# rust-os
在添加 pic8259_simple = "0.2.0" 依赖后，若报 :
`--> /Users/haoyang/.cargo/registry/src/github.com-1ecc6299db9ec823/cpuio-0.3.0/src/lib.rs:4:22
  |
4 | #![feature(llvm_asm, const_fn)]
  |                      ^^^^^^^^ feature has been removed
  |
  = note: split into finer-grained feature gates
`
则将 const_fn 替换为 const_fn_trait_bound