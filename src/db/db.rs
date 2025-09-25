use redis::Commands;



pub fn connet() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;

    let con = client.get_connection()?;

    Ok(con)
}



pub fn add(con: &mut redis::Connection, types: u8, value: String) -> redis::RedisResult<()> {
    let mut key: String = String::new(); 
    if types == 1{
        let file_id: i64 = con.incr("file:counter", 1)?;
        key =  format!("file:{}", file_id);
    }else if types == 2{
        let crawl_id: i64 = con.incr("crawl:counter", 1)?;
        key = format!("crawl:{}", crawl_id);
    }else if types == 3 {
        let crawl_id: i64 = con.incr("visited:counter", 1)?;
        key = format!("visited:{}",crawl_id);
    }

    let _: () = con.set(key, value)?;    
    Ok(())
}


pub fn read(con: &mut redis::Connection, types: u8) -> redis::RedisResult<Vec<String>> {
    let mut keys = Vec::new();
    let mut pattern = String::new();
    if types == 1 { 
        pattern = "file:*".to_string();
    }
    else if types == 2 {    
        pattern = "crawl:*".to_string();
    }else if types == 3 {
        pattern = "visited:*".to_string();
    }

    let iter: redis::Iter<String> = con.scan_match(pattern)?;

    for key in iter {
        keys.push(key);
    }
    Ok(keys)
}


pub fn get(con: &mut redis::Connection, what: String) -> redis::RedisResult<String> {

    let key = con.get(what)?;
    Ok(key)
}




pub fn search(con: &mut redis::Connection, what: String, types: u8) -> redis::RedisResult<bool> {
    let mut pattern = String::new();
    if types == 1 {
        pattern = "file:*".to_string();
    } else if types == 2 {
        pattern = "crawl:*".to_string();
    } else if types == 3 {
        pattern = "visited:*".to_string();
    } else {
        return Ok(false);
    }

    // Raccogli tutte le chiavi prima di iniziare a recuperare i valori
    let keys: Vec<String> = con.scan_match(pattern)?.collect();

    for key in keys {
        let value: String = match con.get(&key) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if value.contains(&what) {
            return Ok(true);
        }
    }
    
    Ok(false)
}