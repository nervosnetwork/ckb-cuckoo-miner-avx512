extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/worker/cuckoo.c")
        .include("src/worker")
        .flag("-O3")
        .flag("-lcrypto")
        .flag("-mavx512f")
        .flag("-mavx512cd")
        .flag("-msse4")
        .static_flag(true)
        .compile("libmean.a");
}