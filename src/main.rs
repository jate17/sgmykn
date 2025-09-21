mod crawler;
use std::collections::HashMap;
use crate::crawler::crawl::crawl_page;
use crate::crawler::utility::check_path_status_url;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

/*
    let mut crawled: HashMap<i32, Vec<String>> = HashMap::new();
    crawled.insert(0, Vec::new());
    crawled.insert(1, Vec::new());

    crawl_page("https://lpg.unibs.it/FI-PPING/", &mut crawled).await;
    println!("{:?}", dizionario);

    crawled.clear() //Puliamo da quelli inseriti


*/


   let f  = String::from("https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf");
   check_path_status_url(f).await;
   Ok(())
}

