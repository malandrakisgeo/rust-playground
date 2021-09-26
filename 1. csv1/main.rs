use std::io;
use std::error::Error;
use serde::Deserialize;
use std::path::Path;
use csv::Reader;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Person{
    id: u16,
    firstname: String,
    lastname: String,
    afm: u32,
    email: String
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut path = String::new();
    let mut afm = String::new();

    io::stdin().read_line(&mut path);
    let mut file = csv::Reader::from_path(Path::new("myFile0.csv"));
    let mut personVec: Vec<Person> = Vec::new();


    for line in file.expect("").deserialize(){
        let prs: Person = line?;
        println!("{:?}", prs);
        personVec.push(prs);
    }

    let mut AFM : u32 ;
    io::stdin().read_line(&mut afm);
    AFM = afm.trim().parse::<u32>().unwrap();

    let index_found = personVec.iter().position(|x| x.afm == AFM );
    println!("{:?}", personVec[index_found.unwrap()]);

    Ok(())


    /*
        1. Pws metatrepoume String apo to io::stdin kateutheian se path?
        2. Pws kanoume persist se mia VD to parapanw?
     */
}