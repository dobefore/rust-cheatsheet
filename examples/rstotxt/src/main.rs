use std::fs;
use walkdir::WalkDir;
use std::time;
///  change extention (a.rs to a.txt) recursively in a folder
/// # example
/// change .rs to .txt
/// ```
/// let suf="rs";
/// let folder="C:\\App";
/// sufix_txt(suf,folder);
/// ```
fn suffix_txt(suf: String, folder: String) {
    // a.rs to a.txt
    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            match entry.path().extension() {
                Some(x) => {
                    let x = format!("{:?}", x);
                    if x.contains(suf.as_str()) {
                        // strip .rs add .txt
                        //   convert str to slice &[]
                        let cr: Vec<_> = suf.chars().collect();
                        let box_slice = cr.into_boxed_slice();
                        let ss = &*box_slice;
                        let trim_rs = entry
                            .path()
                            .to_str()
                            .unwrap()
                            .trim_end_matches(ss)
                            .to_owned();
                        let txt_path = format!("{}{}", trim_rs, "txt");
                        fs::rename(entry.path(), txt_path).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    // let suf = String::from("rs");
    // let folder = String::from(r"D:\software\vscode_project\anki_sync");
    // suffix_txt(suf, folder);
 
}

#[cfg(test)]
mod tests {
    use super::suffix_txt;
    use std::path::Path;

    #[test]
    fn test_box() {
        let s = "ss";
        let v: Vec<_> = s.chars().collect();
        let b = v.into_boxed_slice();
        println!("{:?}", &*b);
    }
    #[test]
    fn test_extension() {
        let s = Path::new("aaa").extension();

        println!("{:?}", s);
    }
}
