extern crate protobuf_codegen_pure;

use std::path::{PathBuf, Path};
use std::fs;

/// Build WE's proto files into Rust sources. They're statically
/// present in the project, but we'll rebuild them on every build
/// to eliminate any possible caching errors and such.
fn main() {
    if Path::new(WE_PROTO_TARGET).exists() {
        let old_generated_src_pattern: &str = &[WE_PROTO_TARGET.clone(), "/*.rs"].concat();
        let old_generated_src: Vec<PathBuf> = glob::glob(old_generated_src_pattern).unwrap().filter_map(Result::ok).collect();

        for old_file in old_generated_src {
            println!("Removing {}", old_file.to_str().unwrap());
            fs::remove_file(old_file).expect("Failed to remove a file");
        }
    } else {
        fs::create_dir(WE_PROTO_TARGET).expect("Failed to create a target directory for proto output");
    }

    let protos_pattern: &str = &[WE_PROTO_SRC_LOCATION.clone(), "/**/*.proto"].concat();

    let protos: Vec<PathBuf> = glob::glob(protos_pattern).unwrap().filter_map(Result::ok).collect();

    protobuf_codegen_pure::Codegen::new()
        .out_dir(WE_PROTO_TARGET)
        .inputs(protos)
        .include(WE_PROTO_SRC_LOCATION)
        .run()
        .expect("Codegen failed.");
}

const WE_PROTO_SRC_LOCATION: &str = "we_proto_src";
const WE_PROTO_TARGET: &str = "src/we_protos";