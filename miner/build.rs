extern crate cc;

fn main() {
    if cfg!(feature="avx2") {
        cc::Build::new()
            .file("src/worker/cuckoo.c")
            .include("src/worker")
            .flag("-O3")
            .flag("-lcrypto")
            .flag("-mavx2")
            .flag("-msse4")
            .static_flag(true)
            .compile("libmean.a");
    } else {
        cc::Build::new()
            .file("src/worker/cuckoo-avx512.c")
            .include("src/worker")
            .flag("-O3")
            .flag("-lcrypto")
            .flag("-mavx512f")
            .flag("-mavx512cd")
            .flag("-msse4")
            .static_flag(true)
            .compile("libmean.a");
    }
}