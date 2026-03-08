# HW 1

To run code in aisa, follow this steps:

```shell
ssh -L58110:andromeda.fi.muni.cz:58110 aisa
export OPENBLAS_TARGET=GENERIC
export OPENBLAS_DYNAMIC_ARCH=0
export OPENBLAS_USE_THREAD=0
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc
module load protobuf-3.6.1
cargo run --bin hw1
```
