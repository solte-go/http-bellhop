use bellhop::files::*;
use bellhop::parser::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut files = vec![];
    let path = Path::new("./requests");
    files::get_files(path, &mut files).expect("TODO: panic message");

    let mut num = 0;

    println!("== Pick config file for request ==");
    for file in &files {
        println!("{:?}. {:?}", num, file.file_name().to_owned());
        num += 1;
    }
    println!("Config:");
    let input = files::get_input();
    let mut req: parser::Request = parser::Request {
        host: String::new(),
        method: String::new(),
        content_type: String::new(),
        body: vec![],
    };
    match input {
        None => {
            println!("Wrong input")
        }
        Some(i) => {
            println!("{:?}", i);
            req = parser::get_cfg_data(&files[i]);
            println!("{:?}", req.body)
        }
    }
    println!("{:?}", req.body);

    // let client = reqwest::Client::builder().build()?;
    let resp = reqwest::Client::new()
        .post(&req.host)
        .json(&req.body)
        .send()
        .await?;
    // .json()
    // .await?;

    println!("{:#?}", resp);
    Ok(())
}
