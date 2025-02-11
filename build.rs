use build_rs_macros as cargo;

const IS_DEV_BUILD: bool = false;

fn main() {
    cargo::rerun_if_changed!("build.rs");
    println!(cargo::rustc_check_cfg!(cfg(dev)));
    if IS_DEV_BUILD {
        println!(cargo::rustc_cfg!(dev));
    }
}
