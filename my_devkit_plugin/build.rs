use std::env;
use std::path::PathBuf;

use cbindgen::Config;

fn main() -> Result<(), &'static str> {

	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let package_name = env::var("CARGO_PKG_NAME").unwrap();
	let output_file = target_dir()
		.join(format!("{}.hpp", package_name))
		.display()
		.to_string();
		

	let config = Config::default();

	cbindgen::generate_with_config(&crate_dir, config)
		.unwrap()
		.write_to_file(&output_file);

    Ok(())
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}