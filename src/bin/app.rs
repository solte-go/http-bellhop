use bellhop::data::files::configs::{get_cfg, get_files};
use bellhop::domain::Request;
use bellhop::model::request;
use std::error::Error;
use std::string::ToString;
use structopt::StructOpt;

const DEFAULT_ENV: &str = "default";

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
    #[structopt(default_value = "http://127.0.0.1:8000", env = "BELLHOP_ADDR")]
    addr: String,
    #[structopt(short, long, help = "what env setup should be used")]
    env: Option<String>,
    #[structopt(short, long, help = "location of json file to run")]
    file: Option<String>,
    #[structopt(short, long, help = "location of dir with json files to run")]
    dir: Option<String>,
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let env;
    if opt.env.is_some() {
        env = opt.env.unwrap();
    } else {
        env = DEFAULT_ENV.to_string();
    }

    if opt.file.is_some() {
        let path = get_cfg(&opt.file.unwrap_or_else(|| "undefined".to_string()))?;
        let req = request::deserialize_data(path.as_path(), &env);
        match req.do_request() {
            Ok(()) => println!("Done"),
            Err(e) => println!("Request filed: {:?}", e),
        }
    } else if opt.dir.is_some() {
        let files = &mut vec![];
        let path = get_cfg(&opt.dir.unwrap_or_else(|| "undefined".to_string()))?;
        let res = get_files(&path, files);
        match res {
            Ok(_) => {
                for file in files {
                    let req = request::deserialize_data(file.as_path(), &env);
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
