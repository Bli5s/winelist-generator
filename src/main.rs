#[macro_use] extern crate rocket;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

let handle = env::var(CELLARTRACKER_USR).unwrap();
let pw = env::var(CELLARTRACKER_PW).unwrap();
let mut inventory = Vec::new();
let mut reader = Reader::from_reader(xml.as_ref());
reader.config_mut().trim_text(true);

struct Wine {
    Size: String,
    Type: String,
    Country: String,
    Region: String,
    Vintage: u8,
    Name: String
}

fn getInventory() -> bool {
    let xml = reqwest::get("https://www.cellartracker.com/xlquery.asp?User={}&Password={}&table=Inventory&format=xml", handle, pw)?.text()?;
    let mut unsupported = false;
    loop {
        if !reader.decoder().encoding().is_ascii_compatible() {
            unsupported = true;
            break;
        }
        
        
    }
    assert_eq!(unsupported, true);
    return true;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

