fn main() {
    prost_build::compile_protos(&["proto/plugin.proto"], &["proto/"]).unwrap();
}
