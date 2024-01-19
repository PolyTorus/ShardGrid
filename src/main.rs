use std::fs;


// ディレクトリの中を再帰的に探索して、ファイル名を表示する
pub fn search_dir(dir: &str) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            search_dir(path.to_str().unwrap());
        } else {
            println!("{}", path.display());
        }
    }
}

fn main() {
    // tmpディレクトリの中身を表示
    search_dir("tmp/");
    
}