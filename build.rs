fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["proto/live-share/live-share.proto"],
            &["proto/live-share"],
        )
        .expect("Could not compile proto files");
}
