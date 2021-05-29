use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// use std::{io,fs,path::Path};
fn copytree<P: AsRef<Path>, Q: AsRef<Path>>(from_dir: P, to_dir: Q) -> io::Result<()> {
    let from_dir = from_dir.as_ref();
    let to_dir = to_dir.as_ref();
    if from_dir.is_dir() && to_dir.is_dir() {
        for i in fs::read_dir(&from_dir)? {
            let i = i?.path();
            let entry_ = &i.file_name().unwrap().to_str().unwrap().to_owned();
            let from_path = i.to_str().unwrap().to_string();
            let to_path = Path::new(to_dir).join(entry_).to_str().unwrap().to_owned();
            if i.is_dir() {
                if !Path::new(&to_path).exists() {
                    fs::create_dir(&to_path)?;
                }
                copytree(&from_path, &to_path)?;
            } else {
                println!("copy file {} please waiting...", &entry_);
                fs::copy(from_path, to_path)?;
            }
        }
    } else {
        panic!("from_dir or _to_dir is not dir");
    }

    Ok(())
}
#[allow(unused_doc_comments)]
fn read_file() {
    /// ```
    ///  with open('foo.txt','w') as f:
    ///     for i in f.readlines() :
    ///         i=i.trim()
    /// ```
    let f = File::open("foo.txt").unwrap();
    let reader = BufReader::new(f);
    for i in reader.lines() {
        println!("{}", i.unwrap());

        // read one line
        /// ```
        ///  with open('foo.txt','w') as f:
        ///     i=f.readline() :
        /// ```
        let f = File::open("foo.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut buf = String::new();
        let byte_num = reader.read_line(&mut buf);

        // read all lines to a string variable
        let mut buf = String::new();
        fs::read_to_string(&mut buf).unwrap();
    }
}
#[allow(unused_doc_comments)]
fn write_file() {
    /// similar to py
    /// ```
    /// with open('foo.txt','w') as f:
    ///      f.write('a\n')
    ///      f.write('b\n')
    ///```
    let mut f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("foo.txt")
        .unwrap();
    writeln!(f, "a");
    writeln!(f, "b");

    /// similar to py
    /// ```
    /// with open('foo.txt','a') as f:
    ///      f.write('a\n')
    ///      f.write('b\n')
    ///```
    let mut f = OpenOptions::new()
        .truncate(true)
        .append(true)
        .open("foo.txt")
        .unwrap();
    writeln!(f, "a");
    writeln!(f, "b");

    // foo,txt just write b after following two line code
    fs::write("foo.txt", b"a").unwrap();
    fs::write("foo.txt", b"b").unwrap();
}
fn main() {}
