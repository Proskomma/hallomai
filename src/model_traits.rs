
trait ScriptureDOM {
    fn init_document() -> String;
}

impl ScriptureDOM for String {
    fn init_document() -> String {
        "{}".to_string()
    }
}