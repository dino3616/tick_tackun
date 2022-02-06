use std::env;
use dotenv::dotenv;
use sqlx::{mysql::MySqlPool,MySql,Pool};

pub struct DataBase{
    pool: Pool<MySql>,
    pub words: Vec<String>,
}

impl DataBase{
    pub async fn new()->Result<DataBase,String>{
        dotenv().ok();
        let database_url=match env::var("DATABASE_URL"){
            Ok(ok)=>ok,
            Err(err)=>{
                return Err(format!("std::env said, {}",err));
            }
        };

        let pool=match MySqlPool::connect(&database_url).await{
            Ok(ok)=>ok,
            Err(err)=>{
                return Err(format!("sqlx said, {}",err));
            }
        };

        Ok(DataBase{
            pool,
            words: Vec::new(),
        })
    }

    pub async fn fetch_all(&mut self)->Result<(),String>{
        let recs=match sqlx::query!(r#"
            SELECT word FROM tingling_words; 
        "#)
            .fetch_all(&self.pool)
            .await{
                Ok(ok)=>ok,
                Err(err)=>{
                    return Err(format!("sqlx said, {}",err));
                }
            };

        self.words.clear();
        for rec in recs{
            self.words.push(rec.word.unwrap());
        }

        Ok(())
    }
}