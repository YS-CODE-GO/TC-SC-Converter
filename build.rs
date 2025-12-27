extern crate winres;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon/icon.ico");
        res.set("CompanyName", "Yssssssss");
        res.set("FileDescription", "专业级中文简繁转换工具");
        res.set("LegalCopyright", "Copyright © 2025 Yssssssss. All rights reserved.");
        res.set("ProductName", "TC-SC Converter");
        res.set("OriginalFilename", "TC-SC-Converter.exe");
        res.set("InternalName", "tc_sc_converter");
        res.compile().unwrap();
    }
}
