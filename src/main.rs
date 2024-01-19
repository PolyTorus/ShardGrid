use std::fs;

fn main() {
    // tmpファイル読み込み
    let entries = fs::read_dir("tmp/").unwrap();

    // ファイル名とサイズを表示
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();

        println!("{} is {} bytes", path.display(), metadata.len());
    }
}