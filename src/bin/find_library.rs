
use std::path::Path;

use lib::find_library;

fn main() {
    let root = "/storage";
    if !Path::new(root).exists() {
        eprintln!("Каталог {} не существует", root);
        return;
    }

    match find_library(root) {
        Ok(subdirs) => {
            for path in subdirs {
                println!("Файл найден: {}", path)
            }
        },

        Err(e) => eprintln!("Ошибка при чтении каталога: {}", e),
    }
}

