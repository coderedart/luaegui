pub fn main() {
    std::fs::DirBuilder::new()
        .create("luaegui_docs")
        .expect("failed to create directory");
    write_type_info_to_file("luaegui_docs/index.md");
}

fn write_type_info_to_file(file_name: &str) {
    let doc = tealr::TypeWalker::new()
        .process_type::<luaegui::Context>()
        .process_type::<luaegui::Ui>()
        .generate("egui", true)
        .expect("failed to generate docs for luaegui");
    std::fs::write(file_name, doc).expect("failed to write to luaegui docs file");
}
