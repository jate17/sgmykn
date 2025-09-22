mod crawler;
mod db; 
use std::collections::HashMap;
use crate::crawler::crawl::crawl_page;
use crate::crawler::utility::check_path_status_url;
use crate::db::db::{connet, read, add};
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let mut crawled: HashMap<i32, Vec<String>> = HashMap::new();
    let mut tocrawl: Vec<String> = Vec::new(); 
    crawled.insert(0, Vec::new());
    crawled.insert(1, Vec::new());

    crawl_page("https://www.cs.unibo.it/~montreso/so/lucidi-so.shtml", &mut crawled).await;
    //println!("{:?}", crawled);
    for links in &crawled{
        for x in links.1 {
            check_path_status_url(x.to_string(),&mut tocrawl).await;
        } 
    }
    println!("{:?}", tocrawl);

    for link in tocrawl {
        crawled.clear();
        crawl_page(&link, &mut crawled).await;
        println!("{:?}", crawled);
    }

    ///crawled.clear() //Puliamo da quelli inseriti

/*


 let f  = String::from("https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf");
   check_path_status_url(f,&mut tocrawl).await;
    
   println!("{:?}", tocrawl);
*/
   Ok(())
}



/*

 match connet() {
        Ok(mut con) => {
            match add(&mut con, 0, "https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf".to_string()){
                Ok(_) => println!("Aggiunto nuovo link"),
                Err(e) => eprintln!("Errore: {:}", e),
            }
            match read(&mut con, 0){
                Ok(value) => println!("{:?}", value),
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Err(e) => {eprintln!("Errore: {:?}", e)}
        
    }
    
*/
