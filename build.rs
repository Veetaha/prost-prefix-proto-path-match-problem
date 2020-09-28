fn main() {
    let mut cfg = prost_build::Config::new();
    cfg.type_attribute(".issue.Foo", "#[derive(Copy)]");
    cfg.compile_protos(&["issue.proto"], &[env!("CARGO_MANIFEST_DIR")]).unwrap();
}
