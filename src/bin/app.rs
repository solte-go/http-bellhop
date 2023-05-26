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
    // #[structopt(default_value = "http://127.0.0.1:8000", env = "BELLHOP_ADDR")]
    // addr: String,
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
            Ok(()) => (),
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
                    match req.do_request() {
                        Ok(()) => (),
                        Err(e) => println!("Request filed: {:?}", e),
                    }
                }
            }
            Err(e) => println!("{:#?}", e),
        }
    }
    println!("-----\nDone");
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e)
    }
}
