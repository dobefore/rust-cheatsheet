use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
/// create dst dir if not exists
fn mkdir(dir: &str) -> PathBuf {
    let dst_dir = Path::new(dir).join("filtered_dir");
    if !dst_dir.exists() {
        fs::create_dir(&dst_dir).unwrap();
    }

    dst_dir
}
/// filter given-extension file  to a new dir
/// from a src dir
/// ### example
/// ```
///let ss = r"D:\BaiduNetdiskDownload";
///    let ext = "mobi";
///    filter_extension(ss, ext);
/// ```
fn filter_extension(src_dir: &str, extension: &str) -> io::Result<()> {
    // mk dst dir
    let root_dir = Path::new(src_dir).parent().unwrap().to_str();
    let dst_dir = mkdir(root_dir.unwrap());
    // walkdir recrusively
    for entry in WalkDir::new(&src_dir) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let entry_path = entry.path();

            // filter extension
            let ext = entry_path.extension();
            let file_name = entry_path.file_name().unwrap().to_str();
            let dst_file_path = Path::new(dst_dir.as_path()).join(file_name.unwrap());
            match ext {
                Some(x) if x == extension => {
                    // cp file to new dir
                    println!(
                        "cp file {} to {}",
                        file_name.unwrap(),
                        &dst_dir.as_path().display()
                    );
                    fs::copy(entry_path, dst_file_path)?;
                }
                None => {}
                _ => {}
            }
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let ss = r"D:\BaiduNetdiskDownload";
    let ext = "mobi";
    filter_extension(ss, ext)?;
    println!("*********done*********");
    Ok(())
}
