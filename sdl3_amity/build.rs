fn main() {
    println!("cargo::rustc-check-cfg=cfg(KHRONOS_USE_INTPTR_T,KHRONOS_SUPPORT_INT64,_WIN64)");
    println!("cargo::rustc-cfg=KHRONOS_USE_INTPTR_T");
    println!("cargo::rustc-cfg=KHRONOS_SUPPORT_INT64");
    if cfg!(all(target_os = "windows", not(target_arch = "x86"))) {
        println!("cargo::rustc-cfg=_WIN64");
    }
}
