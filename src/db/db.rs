use redis::Commands;



pub fn connet() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;

    let con = client.get_connection()?;

    Ok(con)
}



pub fn add(con: &mut redis::Connection, types: u8, value: String) -> redis::RedisResult<()> {
    let mut key: String = String::new(); 
    if types == 0{
        let file_id: i64 = con.incr("file:counter", 1)?;
        key =  format!("file:{}", file_id);
    }else if types == 1{
        let crawl_id: i64 = con.incr("crawl:counter", 1)?;
        key = format!("crawl:{}", crawl_id)
    }

    let _: () = con.set(key, value)?;    
    Ok(())
}


pub fn read(con: &mut redis::Connection, types: u8) -> redis::RedisResult<Vec<String>> {
    let mut keys = Vec::new();
    let mut pattern = String::new();
    if types == 0 { 
        pattern = "file:*".to_string();
    }
    else if types == 1 {    
        pattern = "crawl:*".to_string();
    } 

    let iter: redis::Iter<String> = con.scan_match(pattern)?;

    for key in iter {
        keys.push(key);
    }
    Ok(keys)
}

