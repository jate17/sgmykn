use regex::Regex;
use url::Url;
use std::collections::HashMap;
use std::str;
use reqwest::{self, Client, header};
use std::error::Error;
use crate::db::db::{connet, read, add, get, search};

pub fn append_to_crawl(link: String, typez: i32, wheres: &mut Vec<String>){
    wheres.push(link.to_string())
}

pub fn extract_host(url: &str) -> Option<String> {
    let re_host = Regex::new(
        r"(?i)^(?:[a-z][a-z0-9+.\-]*://)?(?:[^@/]+@)?(\[[0-9a-f:.]+\]|[^/:?#]+)"
    ).unwrap();
    re_host
        .captures(url)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
}


pub fn extract_base_domain(host: &str) -> Option<String> {
    if host.starts_with('[') && host.ends_with(']') {
        return Some(host.to_string());
    }


    let re_two_level = Regex::new(
        r"(?ix)([a-z0-9-]{1,63}\.(?:co|com|org|net|gov|edu|ac|sch|ltd|plc)\.[a-z]{2})$"
    ).unwrap();


    let re_simple = Regex::new(r"(?i)([a-z0-9-]{1,63}\.[a-z0-9-]{2,63})$").unwrap();

    if let Some(c) = re_two_level.captures(host) {
        return c.get(1).map(|m| m.as_str().to_string());
    }
    re_simple
        .captures(host)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
}


pub fn check_url(link: &str, allowed_base_domain: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let re_file = Regex::new(r"(?i)\.(pdf|pptx?|docx?|txt|csv|zip|exe|msi|7z|rar)(?:$|\?)")?;

    let u = Url::parse(link)?;
    let path_lc = u.path().to_ascii_lowercase();

    if path_lc.contains("temi") || path_lc.contains("esame") || path_lc.contains("esami") {
        return Ok(3);
    }

    if re_file.is_match(u.as_str()) {
        return Ok(1);
    }

    let host = u.host_str().unwrap_or("");
    if let Some(host_base) = extract_base_domain(host) {
        if host_base.eq_ignore_ascii_case(allowed_base_domain) {
            return Ok(2);
        } else {
            return Ok(0);
        }
    }
    Ok(0)
}

/*

Da sistemare output aggiungedo i path contrllati ad un Vec

*/


pub async fn analyze_link(target: String, con: &mut redis::Connection)->  Result<(), Box<dyn Error>> {


    /*Ricerca iniziale*/
    
    let val = search(con, target.to_string(), 1)?;
    println!("{:} - {:}", val, target);
    if !val{

        /*Capisce se è un file o no*/
        let re_file = Regex::new(r"\.[a-z]{2,4}(?:\?.*)?$")?;
        if re_file.is_match(&target){
            add(con, 1, target.to_string())?;
        }
        
         /*Divisioe dei vari path*/
        let parti: Vec<&str> = target.split('/').collect();

        for (i,_) in parti.iter().enumerate().take(parti.len() - 3){
            let parts = parti[..parti.len()-i].join("/");
            /*Ricerca le path se è presente*/
            let val = search(con, parts.to_string(), 1)?;
            if !val {
                /*Controlla se è attivo*/
               let status = check_status_url(&parts.to_string()).await;
                if status.0 == 1 && status.1 == "" {
                    add(con, 2, parts.to_string())?; 
                }else if status.0 == 1 && status.1 != "" {
                    add(con, 2, status.1.to_string())?;
                }
            }
        }

       
    }
    Ok(())
}

/*


    Tuple
    (1, "") => 200 o 300/301/302
    (1, string) => 300 + header location
    (2, "") => 404,403,500,timeout, dns problem

*/

async fn check_status_url(url: &String) -> (i32, String) {
    let client = Client::new();

    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();

            if status.is_redirection() {
                match resp.headers().get(header::LOCATION).and_then(|v| v.to_str().ok()) {
                    Some(loc) => return (1,loc.to_string()),
                    None => return (1, "".to_string()),
                }
            } else if status.is_success() {
                return (1, "".to_string())
            } else {
                return (2, "".to_string())
            }
        }
        Err(e) => {
            if e.is_connect() {
                return (2, "".to_string())
            } else if e.is_timeout() {
                return (2, "".to_string())
            } else {
                return (2, "".to_string())
            }
        }
    }
}

