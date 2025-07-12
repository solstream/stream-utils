fn main() {
    tonic_build::compile_protos("proto/shredstream.proto").unwrap();
}
