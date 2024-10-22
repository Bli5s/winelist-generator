#[macro_use] extern crate rocket;
use csv::Reader;
use std::env;
use latex::{print, DocumentClass, Document, Section};

struct Wine {
    size: String,
    variety: String,
    country: String,
    region: String,
    vintage: String,
    name: String,
    grape: String
}

fn get_inventory() -> Vec<Wine> {
    //let handle: String = env::var("CELLARTRACKER_USR").unwrap();
    //let pw: String = env::var("CELLARTRACKER_PW").unwrap();
    let mut inventory: Vec<Wine> = Vec::new();
    //let csv = reqwest::get("https://www.cellartracker.com/xlquery.asp?User={}&Password={}&table=Inventory&format=csv", handle, pw).text_with_charset("utf-8");
    
    let mut reader = Reader::from_path("testdata.csv").unwrap();
    for r in reader.records() {
        match r {
            Ok(r) => {
                inventory.push(Wine{
                    size: r.get(4).unwrap_or("").to_string(),
                    variety: r.get(23).unwrap_or("").to_string(),
                    country: r.get(17).unwrap_or("").to_string(),
                    region: r.get(18).unwrap_or("").to_string(),
                    vintage: r.get(14).unwrap_or("").to_string(),
                    name: r.get(15).unwrap_or("").to_string(),
                    grape: r.get(26).unwrap_or("").to_string()
                });
            }
            Err(e) => {
                println!("Encountered error while parsing csv: {}", e)
            }
        }
    }
    return inventory;
}

fn create_latex(inv: Vec<Wine>) -> Result<String, Error> {
    let mut doc = Document::new(DocumentClass::Article);
    doc.preamble.title("Vinkart");
    doc.preamble.author("Winelistgen");

    for i in inv{
        // DEBUG
        doc.push(Section::new("Red"));
        doc.push(Section::new("White"));
        doc.push(Section::new("Rosé"));
        // END DEBUG

        for e in doc.iter() {
            println!("Section: {:?}", e)
        }
    }
    return print(&doc);
}

#[get("/")]
fn list() {
    let mut inventory = get_inventory();
    let mut doc = create_latex(inventory).unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list])
}