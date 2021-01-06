use std::path::Path;
use tokio::fs;
use tokio::task;
use tokio::time;

async fn A() -> String {
    let sr_ = Path::new(r"a.txt");
    let a = fs::read_to_string(sr_).await.unwrap();
    a
}
async fn B() -> String {
    let sr_ = Path::new(r"b.txt");
    let b = fs::read_to_string(sr_).await.unwrap();
    b
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();

    let aaa = task::spawn(async {
        // must return something
        A().await
    });
    let re = aaa.await;
    println!("{:?}", re);

    let bbb = task::spawn(async {
        // must return something
        B().await
    });
    let ree = bbb.await;
    println!("{:?}", ree);


    let s = now.elapsed().as_secs();
    println!("{}", s);
}
