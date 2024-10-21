#[macro_use] extern crate rocket;
extern crate csv;
use std::env;

struct Wine {
    size: String,
    variety: String,
    country: String,
    region: String,
    vintage: u8,
    name: String
}

fn get_inventory() -> bool {
    let handle: String = env::var("CELLARTRACKER_USR").unwrap();
    let pw: String = env::var("CELLARTRACKER_PW").unwrap();
    let mut inventory: Vec<Wine> = Vec::new();

    //let csv = reqwest::get("https://www.cellartracker.com/xlquery.asp?User={}&Password={}&table=Inventory&format=csv", handle, pw).text();
    
    let mut reader = csv::Reader::from_path("testdata.csv").unwrap();
    for r in reader.records() {
        match r {
            Ok(r) => println!("{:?}", r),
            Err(e) => {
                println!("Encountered error while parsing csv: {}", e) 
            }
        }
    }
    
    return true;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn list() {
    get_inventory();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![list])
}

