use std::env;
use std::path::Path;

use lib::find_archives;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <library_root> <book_id>", args[0]);
        return;
    }

    let root = &args[1];
    let book: u32 = match args[2].parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Invalid id. It should be an integer.");
            return;
        }
    };

    if !Path::new(root).exists() {
        eprintln!("Каталог {} не существует", book);
        return;
    }

    match find_archives(root, book) {
        Ok(archives) => {
            if archives.is_empty() {
                println!("Нет архивов, содержащих диапазон, включающий id {book}");
            } else {
                println!("Архивы, содержащие диапазон, включающий id {book}:");
                for archive in archives {
                    println!("{}", archive.display());
                }
            }
        }
        Err(e) => eprintln!("Ошибка при чтении каталога: {}", e),
    }
}
