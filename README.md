# filemanager-rs 📟

Simple cli file manager written in Rust without any external dependencies. Only standard library.

# Usage 📨:

1. Clone the repo: `git clone https://github.com/Hkmori15/filemanager-rs.git`
2. Compile the program: `cargo build --release`
3. For viewing list of files in the directory: `cargo run list "directory"`
4. For copy file: `cargo run copy source.txt dest.txt`
5. For move file: `cargo run move source.txt dest.txt`
6. For delete file: `cargo run delete source.txt`
7. For recursing viewing list files in the directory: `cargo run list_recursive "directory"`
8. For recursing viewing list files in the directory + extenion: `cargo run list_recursive "directory" --extension txt`
9. For recursing viewing list files in the directory + filename: `cargo run list_recursive "directory" --name readme`
10. For recursing viewing list files in the directory + extension + filename: `cargo run list_recursive "directory" --extension md --name readme`
11. For symlink file: `cargo run symlink source.txt link.txt` or `cargo run symlink dir_source dir_link`
12. For change permissions file/directory using chmod: `cargo run -- chmod 777 "directory" or "file"`
13. For testing all func filemanager-rs: `cargo test`
