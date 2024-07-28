
use std::path::Path;

use opds_tools::find_libraries;

fn main() {
    let root = "/storage";
    if !Path::new(root).exists() {
        eprintln!("Каталог {} не существует", root);
        return;
    }

    match find_libraries(root) {
        Ok(subdirs) => {
            for path in subdirs {
                println!("Файл найден: {}", path)
            }
        },

        Err(e) => eprintln!("Ошибка при чтении каталога: {}", e),
    }
}

