const DIRECTORY_NAME: &str = "luaegui_docs";
const DOCS_FILE_PATH: &str = "luaegui_docs/index.md";

pub fn main() {
    let doc = write_type_info_to_file();

    // to embed it all cleanly within a codeblock
    let doc = format!("```\n{}\n```", doc);

    std::fs::DirBuilder::new()
        .create(DIRECTORY_NAME)
        .expect("failed to create directory");

    std::fs::write(DOCS_FILE_PATH, doc).expect("failed to write to luaegui docs file");
}

fn write_type_info_to_file() -> String {
    tealr::TypeWalker::new()
        .process_type::<luaegui::Ui>()
        .process_type::<luaegui::Context>()
        .process_type::<luaegui::Response>()
        .generate("egui", true)
        .expect("failed to generate docs for luaegui")
}
