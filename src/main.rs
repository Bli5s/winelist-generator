#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use failure;
use csv::Reader;
use std::{collections::HashSet, env, path::PathBuf};
use latex::{print, DocumentClass, Document, Section};
use pandoc::{self, Pandoc};

#[derive(Eq, Hash, PartialEq)]
struct Wine {
    size: String,
    variety: String,
    country: String,
    region: String,
    vintage: String,
    name: String,
    grape: String
}

fn get_inventory() -> HashSet<Wine> {
    //let handle: String = env::var("CELLARTRACKER_USR").unwrap();
    //let pw: String = env::var("CELLARTRACKER_PW").unwrap();
    let mut inventory: HashSet<Wine> = HashSet::new();
    //let csv = reqwest::get("https://www.cellartracker.com/xlquery.asp?User={}&Password={}&table=Inventory&format=csv", handle, pw).text_with_charset("utf-8");
    
    let mut reader = Reader::from_path("testdata.csv").unwrap(); // TEST

    for r in reader.records() {
        match r {
            Ok(r) => {
                inventory.insert(Wine{
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

fn create_latex(inv: &HashSet<Wine>) -> Result<String, failure::Error> {
    let mut doc = Document::new(DocumentClass::Article);
    doc.preamble.title("Vinkart");
    doc.preamble.author("Winelistgen");
    let mut varieties: HashSet<String> = HashSet::new();

    for i in inv{
        varieties.insert(i.variety.clone());
    }

    for v in varieties{
        let mut s = Section::new(&v);
        for w in inv{
            let vintage = if w.vintage == "1001" {
                "N.V."
            } else {
                w.vintage.as_str()
            }.to_string();
            if w.variety == v {
                s.push(format!("\\subsection{{{}, {}}}", w.name, vintage).as_str());
                s.push(format!("{}, {}, {}, {}", w.country, w.region, w.grape, w.size).as_str());
            }
        }
        doc.push(s);
    }

    return print(&doc);
}

fn pdf() -> String {
    let file_path = "static/winelist.pdf".to_string();
    let inventory = get_inventory();
    let doc = create_latex(&inventory).unwrap();
    let mut pandoc = Pandoc::new();
    pandoc.set_input_format(pandoc::InputFormat::Latex, Vec::new());
    pandoc.set_output_format(pandoc::OutputFormat::Pdf, Vec::new());
    pandoc.set_input(pandoc::InputKind::Pipe(doc));
    pandoc.set_output(pandoc::OutputKind::File(PathBuf::from(&file_path)));
    pandoc.execute().unwrap();
    return file_path;
}

#[get("/")]
fn html() -> String {
    let inventory = get_inventory();
    let doc = create_latex(&inventory).unwrap();
    let mut pandoc = Pandoc::new();
    pandoc.set_input_format(pandoc::InputFormat::Latex, Vec::new());
    pandoc.set_output_format(pandoc::OutputFormat::Html5, Vec::new());
    pandoc.set_input(pandoc::InputKind::Pipe(doc));
    pandoc.set_output(pandoc::OutputKind::Pipe);
    pandoc.execute().unwrap();
    return String::from("Html generation done!\n");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![html])
        .mount("/pdf", FileServer::from(pdf()))
}