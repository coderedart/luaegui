use tealr::TypeWalker;

const DOCS_FILE_PATH: &str = "luaeguidocs.json";

pub fn main() {
    let type_walker = write_type_info_to_file();

    // to embed it all cleanly within a codeblock
    let doc = serde_json::to_string(&type_walker).expect("failed serialize type walker");

    std::fs::write(DOCS_FILE_PATH, doc).expect("failed to write to luaegui docs file");
}

fn write_type_info_to_file() -> TypeWalker {
    use luaegui::*;
    tealr::TypeWalker::new()
        .process_type::<Ui>()
        .process_type::<Context>()
        .process_type::<Response>()
        .process_type::<Color32>()
        .process_type::<RichText>()
        .process_type::<EguiProxy>()
        .process_type::<WidgetText>()
        .process_type::<Galley>()
}
