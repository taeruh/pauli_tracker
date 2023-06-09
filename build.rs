fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        println!(r#"cargo:rustc-cfg=target_feature="avx2""#);
    }
}
