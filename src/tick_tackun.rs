use std::process;
use super::sql::*;

pub struct TickTackun{
    pub hit_words: Vec<String>,
}

impl TickTackun{
    pub async fn is_tingling_words(_words: &String)->bool{
        let mut database=match DataBase::new().await{
            Ok(ok)=>ok,
            Err(err)=>{
                eprint!("Error: {}\n",err);
                process::exit(1);
            }
        };
        if let Err(err)=database.fetch_all().await{
            eprint!("Error: {}\n",err);
            process::exit(1);
        };

        print!("{:?}\n",database.words);

        true
    }
}