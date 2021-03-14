# oci-lib
Oci-Spec for your container runtime or container registry.
- - -
Oci-lib is a rust port for original oci spec written in go. Following crate contains rust equivalent spec for OCI Image Format Specification ( https://github.com/opencontainers/image-spec ) and OCI Runtime format specification (https://github.com/opencontainers/runtime-spec).

## Motivation
Initially written as a dependency for vas-quod ( https://github.com/flouthoc/vas-quod ). 

## Usage
```rust
extern crate oci_lib;
use oci_lib::runtime;
use oci_lib::image;

fn main() {
    let runtime_spec = match runtime::Spec::load("path") {
        Ok(spec) => spec,
        Err(e) => panic!("{}", e),
    }
    let image_spec = match image::ImageConfig::load("path") {
        Ok(spec) => spec,
        Err(e) => panic!("{}", e),
    }
}
```
### References
* https://opencontainers.org/
* https://github.com/opencontainers/image-spec/blob/master/specs-go/v1/config.go
* https://github.com/opencontainers/runtime-spec/blob/master/specs-go/config.go
* https://github.com/oracle/railcar/tree/master/oci
