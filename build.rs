fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .expect("Could not compile proto files");
}