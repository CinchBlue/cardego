use std::{
    env,
    path::{Path, PathBuf},
};

fn build_target_is_wasm() -> bool {
    let target: String = std::env::var("TARGET").unwrap_or_default();
    let is_wasm = target.contains("wasm");
    println!("is_build_target_wasm? {:?}", is_wasm);
    is_wasm
}

fn _build_feature_client() -> bool {
    let feature_client: String = std::env::var("CARGO_FEATURE_CLIENT").unwrap_or_default();
    let build_client = feature_client == "true";
    println!("build_client? {:?}", build_client);
    build_client
}

fn _build_feature_server() -> bool {
    let feature_server: String = std::env::var("CARGO_FEATURE_SERVER").unwrap_or_default();
    let build_server = feature_server == "true";
    println!("build_server? {:?}", build_server);
    build_server
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let out_dir = "./src/generated";

    let current_dir = env::current_dir().unwrap();
    let repo_root_dir = current_dir.clone();
    let proto_path = Path::new(&repo_root_dir).join("proto/");

    println!("current_dir: {:?}", current_dir);
    println!("proto_path: {:?}", proto_path);
    let include_paths: Vec<PathBuf> = vec![current_dir];

    let proto_files = vec![proto_path.join("cardego-data-service.proto")];

    tonic_build::configure()
        // .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("cardego-data-service.bin"))
        .build_transport(!build_target_is_wasm())
        .compile(proto_files.as_slice(), &include_paths)?;
    Ok(())
}
