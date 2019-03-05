fn main() {
    prost_build::compile_protos(
        &[
            "proto/plugin.proto",
            "proto/client.proto",
            "proto/common.proto",
        ],
        &["proto/"],
    )
    .unwrap();
}
