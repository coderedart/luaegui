const DOCS_FILE_PATH: &str = "luaeguidocs.json";

pub fn main() {
    let type_walker = luaegui::get_all_types();

    // to embed it all cleanly within a codeblock
    let doc = serde_json::to_string(&type_walker).expect("failed serialize type walker");

    std::fs::write(DOCS_FILE_PATH, doc).expect("failed to write to luaegui docs file");
}
