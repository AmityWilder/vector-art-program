use build_rs_macros as cargo;
use copy_to_output::copy_to_output;

const IS_DEV_BUILD: bool = false;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!(cargo::rustc_check_cfg!(cfg(dev)));
    if IS_DEV_BUILD {
        println!(cargo::rustc_cfg!(dev));
    }
    println!("cargo::rerun-if-changed=src/assets");
    copy_to_output("src/assets", &std::env::var("PROFILE").unwrap()).expect("Could not copy");
}
