use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(graphics_api, values(\"opengl_11\",\"opengl_21\",\"opengl_33\",\"opengl_43\",\"opengl_es2\",\"opengl_es3\"))");
    if env::var("graphics_api").is_err() {
        println!("cargo::rustc-check-cfg=graphics_api=\"opengl_33\"");
    }
}
