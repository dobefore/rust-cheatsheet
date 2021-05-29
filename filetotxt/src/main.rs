use walkdir::WalkDir;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
/// ```
/// let sr=r"G:\folder";
/// let ds=".\\filelist.txt";
/// filetotxt(sr,ds)
/// ```
fn filetotxt(folder:&str,DstTxtPath:&str) {
// walk dir
for entry in WalkDir::new(folder){
    let entry = entry.unwrap();
    if entry.path().is_file() {
    let filename=entry.path().file_name().unwrap().to_str().unwrap().to_owned();
// write filename to txt by lines
let mut file = OpenOptions::new().create(true).append(true).open(DstTxtPath).unwrap();
writeln!(file,"{}",filename).unwrap();
    
    }
}


}

fn search_file_from_txt(filepath:&str,filename:&str) {
    let f = File::open(filepath).unwrap();
    let lowfn=filename.to_ascii_lowercase();
    let mut reader = BufReader::new(f);
    
    let mut match_res=vec![];
    loop  {
        let mut s=String::new();
     let num_bytes=   reader.read_line(&mut s).unwrap();
     let lows=s.to_ascii_lowercase();
        if lows.contains(&lowfn) {
            match_res.push(s);
        }
        if num_bytes==0 {break;}
    };
    println!("matched results: {}",&match_res.len());
    if match_res.len()!=0 {
       for i in match_res{
           println!("{}",i);
       }
    }
}
fn main() {
    let sr=r"";
    let ds=".\\filelist.txt";
    // filetotxt(sr, ds);
    search_file_from_txt(ds, "周恩来");
}

#[test]
fn test_contain(){
    let s="Afgvv";
    println!("{}",s.contains("af")); //false
}