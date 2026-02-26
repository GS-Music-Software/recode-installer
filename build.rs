fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target == "windows" {
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
