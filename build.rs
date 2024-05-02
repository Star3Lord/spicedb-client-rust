fn main() {
    let proto_files: Vec<std::path::PathBuf> = glob::glob("proto/**/*.proto")
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    tonic_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .file_descriptor_set_path(&descriptor_path)
        .include_file("mod.rs")
        // .out_dir("src/proto_build")
        .compile(&proto_files, &["proto"])
        .unwrap();
}
