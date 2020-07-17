fn main() {
    cc::Build::new()
        .file("vendor/MemoryModule/MemoryModule.c")
        .compile("MemoryModule");
}
