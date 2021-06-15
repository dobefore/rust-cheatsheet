use std::path::Path;
use rusqlite::{Connection, NO_PARAMS,};
pub use rusqlite::{Result,params,RowIndex,types::FromSql,Row};
pub struct Sqlite{
 pub   db: Connection
}
impl Sqlite {
   pub  fn new_conn<P:AsRef<Path>>(path:P) ->Result<  Self>{
     Ok(  Self{ db: Connection::open(path)?})
      
    }
    /// query one row
    pub fn fetchone<T:FromSql>(&self, sql: &str) -> Result<T> {
   let s=   self.db
            .query_row(sql, NO_PARAMS, |r| r.get(0))?;
    Ok(s)
    }  
   pub  fn search_fetcball<T>(&self,sql: &str)->Result<Vec<T>>
    where
    
    T:FromSql
     {
        let mut stmt=self.db.prepare(sql)?;
        let rows = stmt.query_map(params![], |row| row.get(0))?;

    let mut items:Vec<T> = Vec::new();
    for name_result in rows {
        items.push(name_result?);
    }
    Ok(items)
    }
    pub fn fetchall<I:RowIndex+Copy,T:FromSql>(&self,sql: &str,index_of_column:I)->Result<Vec<T>> {
        let mut s= self.db.prepare(sql)?;
        let mut  rows= s.query(params![])?;
        let mut items:Vec<T> = vec![];
        while let Some(row) = rows.next()? {
            items.push(row.get(index_of_column).unwrap());
        }
        Ok(items)
    }
    pub fn db_execute_many(
        &self,
        sql: &str,
        args: Vec<[String;3]>,
    ) -> Result<()> {
        let mut stmt = self.db.prepare(sql)?;
    
        for param in args {
    
            println!("{}",param[0]);
          let n=  stmt.execute(params![param[0],param[1],param[2]])?;
        }
    
        Ok(())
    }
}

