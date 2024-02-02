fn main() {
    println!("cargo:rerun-if-changed=src/api/proto");
    println!("cargo:rerun-if-changed=../../proto/config.proto");

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("./src/api/proto/")
        .compile(&["../../proto/config.proto"], &["../../proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
