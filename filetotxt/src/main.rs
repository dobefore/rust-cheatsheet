
use std::fs::{File, write};
use std::fs::{self, OpenOptions};
use std::io::{self, prelude::*};
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;
use rusqlite;
mod sql;
use sql::*;
/// ```
/// let sr=r"G:\folder";
/// let ds=".\\filelist.txt";
/// filetotxt(sr,ds)
/// ```
fn filetotxt(folder: &str, DstTxtPath: &str) {
    let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(DstTxtPath)
                .unwrap();
    // walk dir
    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let filename = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            // write filename to txt by lines
            writeln!(file, "{}", filename).unwrap();
        }
    }
}

fn search_file_from_txt(filepath: &str, filename: &str) {
    let f = File::open(filepath).unwrap();
    let lowfn = filename.to_ascii_lowercase();
    let mut reader = BufReader::new(f);

    let mut match_res = vec![];
    loop {
        let mut s = String::new();
        let num_bytes = reader.read_line(&mut s).unwrap();
        let lows = s.to_ascii_lowercase();
        if lows.contains(&lowfn) {
            match_res.push(s);
        }
        if num_bytes == 0 {
            break;
        }
    }
    println!("matched results: {}", &match_res.len());
    if match_res.len() != 0 {
        for i in match_res {
            println!("{}", i);
        }
    }
}
fn return_bookname_path<P:AsRef<Path>>(dir:P,mut init_number:usize) ->Vec<[String;3]>{
    let mut v=vec![];
   
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            init_number+=1;
            let filename = entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
            let filepath=entry.path().to_str().unwrap().to_owned();
            v.push([init_number.to_string(),filename,filepath]);
        }
    }
    v
}
type Result<T>=std::result::Result<T,SError>;
#[derive(Debug)]
enum SError {
    Ior(io::Error),
    Sql(rusqlite::Error)
}
fn cp_chosenfile_dstdir<P:AsRef<Path>,Q:AsRef<Path>>(keyword:&str,dbpath:P,dstdir:Q) ->Result<()>{
    // search file 
    let conn = Sqlite::new_conn(dbpath)?; 
    let sql0=format!( r"select bookname
    from bookpath 
    where bookname 
    LIKE '%{}%'",keyword);
   let v:Vec<String>= conn.search_fetcball(&sql0)?;
    
//    iter fnames
if !v.is_empty() {
    let mut n=0;
    for item in &v {
        // print results
println!("{}--{}",&n,item);
n+=1;
    }
    // input number to get fname from v
    print!("input number to choose:");
    io::stdout().flush().unwrap();
    let mut ip=String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let  ip=ip.trim().parse::<usize>().unwrap();
let fname=v.get(ip).unwrap().to_owned();
// lookup filepath in db where fname==s
let sql=format!( "SELECT bookpath from bookpath where bookname='{}'",fname);
let bkpath:String=conn.fetchone(&sql)?;
// cp file
println!("{}",&bkpath);
let dstpath=Path::new(dstdir.as_ref()).join(fname);
fs::copy(bkpath, dstpath).unwrap();
}else {
    println!("No Items Found")
}
   Ok(())
}
fn read_bookname_path_into_db<P:AsRef<Path>,Q:AsRef<Path>>(dir:P,dbpath:Q)->Result<()>{
    // check if table exists
    if !dbpath.as_ref().exists() {
         // create db if not exist.
    // ..a.db3/.db
        let    conn = Sqlite::new_conn(&dbpath)?;
    conn.db.execute_batch ("CREATE TABLE bookpath (
        id integer PRIMARY KEY UNIQUE,
        bookname text NOT NULL,
        bookpath  text NOT NULL
      );").unwrap();
    }
  let   conn = Sqlite::new_conn(dbpath)?;
//   get max id 
let sql1="SELECT id from bookpath";
let mut v1:Vec<i64>= conn.fetchall(sql1, 0)?;

let  init_num= if v1.is_empty() {
   0
}else{
    v1.pop().unwrap()
};

 // walk dir to get bookname and its absolute path(first path item if many)
 let v=return_bookname_path(dir,init_num as usize);

    // add bookname and its absolute path into db
    let sql="INSERT INTO bookpath VALUES (?,?,?)";
 conn.db_execute_many(sql, v)?;
conn.db.close().unwrap();
    Ok(())
  
}
fn main() {
    let sr = r"H:\book";
    let ds = ".\\filelist.txt";
    // filetotxt(sr, ds);
    // search_file_from_txt(ds, "我的公关人生");
// read_bookname_path_into_db(sr, "dbpath.db").unwrap();
cp_chosenfile_dstdir("后浪", "dbpath.db", ".").unwrap();
}

#[test]
fn test_contain() {
    let s = "Afgvv";
    println!("{}", s.contains("af")); //false
}
#[test]
fn test_write_file() {
    let p = "f.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(p)
        .unwrap();

        writeln!(file,"a").unwrap();
        writeln!(file,"b").unwrap();
        write!(file, "{}\r\n","c").unwrap();

    }


    #[test]
    fn test_search_db() {
        let dbpath="..\\dbpath.db";
        let keyword="写作";
        let conn = Sqlite::new_conn(dbpath).unwrap(); 
        let sql0=format!( r"select bookname
        from bookpath 
        where bookname 
        LIKE '%{}%'",keyword);
       let v:Vec<String>= conn.search_fetcball(&sql0).unwrap();  
       for i in &v {
           println!("{}",i);
       } 
}