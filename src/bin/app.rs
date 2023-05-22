use bellhop::files::*;
use bellhop::parser::parser::{Method, Request};
use bellhop::parser::*;
use std::error::Error;
use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// enum Command {
//     File{
//         #[structopt(short, long, help = "location of json file to run")]
//         file: Option<String>,
//     },
//     Dir {
//         #[structopt(short, long, help = "location of dir with json files to run")]
//         dir: Option<String>,
//     },
//     // Default{
//     //     default_request: bool,
//     // }
// }

#[derive(StructOpt, Debug)]
#[structopt(name = "http-bellhop", about = "HTTP Bellhop CLI tool for API testing")]
struct Opt {
    // #[structopt(subcommand)]
    // command: Command,
    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSTASH_ADDR")]
    addr: String,
    #[structopt(short, long, help = "location of json file to run")]
    file: Option<String>,
    #[structopt(short, long, help = "location of dir with json files to run")]
    dir: Option<String>,
}

fn make_request(req: Request) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    match req.method {
        Method::GET => {
            let mut request = client.get(&req.host);
            let r = request.json(&req.body).send()?;
            println!("{:#?}", r)
        }

        Method::POST => {
            let mut request = client.post(&req.host);
            let r = request.json(&req.body).send()?;
            println!("{:#?}", r)
        }
        _ => println!("wrong argument"),
    }

    // if req.method.to_lowercase() == "post" {
    //     let mut request = client.post(&req.host);
    //     let r = request.json(&req.body).send()?;
    //     println!("{:#?}", r)
    // }
    // if req.method == "get" {
    //     let mut request = client.get(&req.host);
    //     let r = request.json(&req.body).send()?;
    //     println!("{:#?}", r)
    // }
    Ok(())
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    if opt.file.is_some() {
        let path = files::get_cfg(&opt.file.unwrap_or_else(|| "undefined".to_string()))?;
        let req = parser::get_cfg_data(path.as_path());
        make_request(req).expect("err");
    } else if opt.dir.is_some() {
        let files = &mut vec![];
        let path = files::get_cfg(&opt.dir.unwrap_or_else(|| "undefined".to_string()))?;
        let res = files::get_files(&path, files);
        match res {
            Ok(_) => {
                for file in files {
                    let req = parser::get_cfg_data(file.as_path());
                }
            }
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}

// #[tokio::main]

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e)
    }

    // let mut files = vec![];
    // let path = Path::new("./requests");
    // files::get_files(path, &mut files).expect("TODO: panic message");
    //
    // let mut num = 0;
    //
    // println!("== Pick config file for request ==");
    // for file in &files {
    //     println!("{:?}. {:?}", num, file.file_name().to_owned());
    //     num += 1;
    // }
    // println!("Config:");
    // let input = files::get_input();
    // let mut req= Request::default();
    //
    // match input {
    //     None => {
    //         println!("Wrong input")
    //     }
    //     Some(i) => {
    //         println!("{:?}", i);
    //         req = parser::get_cfg_data(&files[i]);
    //         println!("{:?}", req.body)
    //     }
    // }
    // println!("{:?}", req.body);
    //
    // // let client = reqwest::Client::builder().build()?;
    // let resp = reqwest::Client::new()
    //     .post(&req.host)
    //     .json(&req.body)
    //     .send()
    //     .await?;
    // // .json()
    // // .await?;
    //
    // println!("{:#?}", resp);
    // Ok(())
}
