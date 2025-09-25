mod crawler;
mod db; 
use std::collections::HashMap;
use crate::crawler::crawl::crawl_page;
use crate::crawler::utility::analyze_link;
use std::str;
use crate::db::db::{connet, read, add, get, search};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    

    let test: [&str; 58] = ["https://www.cs.unibo.it/~montreso/so/index.shtml", "https://www.cs.unibo.it/~montreso/so/orario.shtml", "https://www.cs.unibo.it/~montreso/so/orario.shtml#ricevimento", "https://www.cs.unibo.it/~montreso/so/calendario.shtml", "https://www.cs.unibo.it/~montreso/so/iscrizioni.shtml", "https://www.cs.unibo.it/~montreso/so/regolamento.shtml", "https://www.cs.unibo.it/~montreso/so/faq.shtml", "https://www.cs.unibo.it/~montreso/so/programma-so.shtml", "https://www.cs.unibo.it/~montreso/so/lucidi-so.shtml", "https://www.cs.unibo.it/~montreso/so/compiti-so.shtml", "https://www.cs.unibo.it/~montreso/so/programma-lso.shtml", "https://www.cs.unibo.it/~montreso/so/lucidi-lso.shtml", "https://www.cs.unibo.it/~montreso/so/esercitazioni-lso.shtml", "https://www.cs.unibo.it/~montreso/so/provapratica-lso.shtml", "https://www.cs.unibo.it/~montreso/so/progetto-lso.shtml", "https://www.cs.unibo.it/~montreso/so/libri.shtml", "https://www.cs.unibo.it/~montreso/so/docs.shtml", "https://www.cs.unibo.it/~montreso/doc/papers/", "https://www.cs.unibo.it/~montreso/so/lucidi/so-01-intro-corso.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-01-intro-corso-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-01-intro-corso-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-01-intro-corso-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-02-intro-os.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-02-intro-os-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-02-intro-os-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-02-intro-os-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-03-concorrenza.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-03-concorrenza-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-03-concorrenza-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-03-concorrenza-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-04-struttura-os.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-04-struttura-os-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-04-struttura-os-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-04-struttura-os-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-05-scheduling.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-05-scheduling-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-05-scheduling-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-05-scheduling-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-06-risorse.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-06-risorse-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-06-risorse-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-06-risorse-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-07-memoria.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-07-memoria-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-07-memoria-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-07-memoria-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-08-secondaria.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-08-secondaria-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-08-secondaria-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-08-secondaria-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-09-filesystem.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-09-filesystem-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-09-filesystem-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-09-filesystem-4p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-10-windows2000.sxi", "https://www.cs.unibo.it/~montreso/so/lucidi/so-10-windows2000-1p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-10-windows2000-2p.pdf", "https://www.cs.unibo.it/~montreso/so/lucidi/so-10-windows2000-4p.pdf"];
    let mut con = connet()?;
    let mut target = "https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf";
    
    /*
    for link in &test{
    
        analyze_link(link.to_string(), &mut con).await;
    }
*/

   

   let t = read(&mut con, 1);
   let f = read(&mut con, 2);
    println!("{:?}", t);
    println!("{:?}", f);
  
   Ok(())
}



/*


//let mut crawled: Vec<String> = Vec::new();
    //let mut tocrawl: Vec<String> = Vec::new(); 
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


/* 

   match connet() {
        Ok(mut con) => {
             match add(&mut con, 1, "https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf".to_string()){
                Ok(_) => println!("Aggiunto nuovo link"),
                Err(e) => eprintln!("Errore: {:}", e),
            }
            /*
            match read(&mut con, 1){
                Ok(value) => println!("{:?}", value),
                Err(e) => eprintln!("{:?}", e),
            }*/
            match get(&mut con, "file:3".to_string()){
                Ok(value) => println!("{:?}", value),
                Err(e) => eprintln!("{:?}", e)
            }
        },
        Err(e) => {eprintln!("Errore: {:?}", e)}
        
    }
*/
 

 
/*
   let f  = String::from("https://docs.univr.it/documenti/OccorrenzaIns/matdid/matdid345963.pdf");
   check_path_status_url(f,&mut tocrawl).await;
    
   println!("{:?}", tocrawl);


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



*/