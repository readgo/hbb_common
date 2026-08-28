fn main() {
    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");

    // yy: 多版本独立配置目录 —— 注入 YY_APP_NAME（各版本不同，
    // 决定 ~/.config/<APP_NAME>/ 配置目录，避免收藏/历史/服务器互覆盖）
    if let Ok(app_name) = std::env::var("YY_APP_NAME") {
        if !app_name.is_empty() {
            println!("cargo:rustc-env=YY_APP_NAME={}", app_name);
            println!("cargo:warning=yy: app name: {}", app_name);
        }
    }
    println!("cargo:rerun-if-env-changed=YY_APP_NAME");
}
