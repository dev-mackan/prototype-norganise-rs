mod nvim;
pub use nvim::NvimEditor;

pub trait TextEditor {
    fn open_temp_file(text: &str) -> anyhow::Result<String>;
}
