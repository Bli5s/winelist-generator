#[macro_use]
extern crate rocket;
use rocket::fs::{relative, NamedFile};
use failure;
use csv::Reader;
use std::{collections::HashSet, env, path::PathBuf};
use latex::{print, Document, DocumentClass, Paragraph, Section};
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

const DOC_TITLE: &str = "Wine list";
const PDF_PATH: &str = relative!("static/winelist.pdf");

async fn get_inventory() -> HashSet<Wine> {
    let handle: String = env::var("CELLARTRACKER_USR").unwrap();
    let pw: String = env::var("CELLARTRACKER_PW").unwrap();
    let mut csv: String = String::new();
    let mut inventory: HashSet<Wine> = HashSet::new();
    let req = reqwest::get(format!("https://www.cellartracker.com/xlquery.asp?User={}&Password={}&table=Inventory&format=csv", handle, pw)).await;
    match req {
        Ok(r) => {
            csv = r.text_with_charset("utf-8").await.unwrap();
        }
        Err(e) => {
            println!("HTTP GET returned an error: {}", e)
        }
    }
    
    let mut reader = Reader::from_reader(csv.as_bytes());

    for r in reader.records() {
        match r {
            Ok(r) => {
                inventory.insert(Wine{
                    size: r.get(4).unwrap().to_string(),
                    variety: r.get(23).unwrap().to_string(),
                    country: r.get(17).unwrap().to_string(),
                    region: r.get(18).unwrap().to_string(),
                    vintage: r.get(14).unwrap().to_string(),
                    name: r.get(15).unwrap().to_string(),
                    grape: r.get(26).unwrap().to_string()
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
    doc.preamble.title(DOC_TITLE);
    let mut varieties: HashSet<String> = HashSet::new();

    for i in inv{
        varieties.insert(i.variety.clone());
    }

    for v in varieties{
        let mut pb = Paragraph::new();
        pb.push("\\pagebreak");
        doc.push(pb);
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

async fn generate_pdf() {
    let inventory = get_inventory().await;
    let doc = create_latex(&inventory).unwrap();
    let mut pandoc = Pandoc::new();
    pandoc.set_input_format(pandoc::InputFormat::Latex, Vec::new());
    pandoc.set_output_format(pandoc::OutputFormat::Latex, Vec::new());
    pandoc.set_input(pandoc::InputKind::Pipe(doc));
    pandoc.set_output(pandoc::OutputKind::File(PathBuf::from(String::from(PDF_PATH))));
    pandoc.execute().unwrap();
}

#[get("/")]
async fn serve_pdf() -> Option<NamedFile> {
    generate_pdf().await;
    NamedFile::open(PDF_PATH).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![serve_pdf])
}