
A simple CLI-tool to organise notes. This is just a prototype, where I try my ideas out for this tool.

It is in _very_ early development stages. 
The application is only tested on Linux.

Try the application using `cargo run`

**Dependencies**

The application uses `fzf` to perform fuzzy searching.
`Neovim` is the editor used to open and edit the contents of notes.

The configuration and storage files are created in `~/.config/norganise-rs/`.
The path for the storage json can be changed by editing the `data_file_path` value in `config.json`.

**Keybindings**

|Description|Key|
|-------------|---|
|Previous item|`k`|
|Next Item|`j`|
|Edit note info|`e`|
|New Note|`n`|
|Delete Note|`d`|
|Open note|`<Enter>`| 
|Search|`/`|
|Close note editor/popup|`<Esc>`|
|Next sort mode|`s`|
|Prev sort mode|`<Shift>+s`|
|Next field in a popup|`<Tab>`|
|Open item list for a popup|`<Ctrl>+<Space>`|
|Quit|`q`|



