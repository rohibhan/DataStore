use clap::{Parser, ValueEnum};
use kvs::KvStore;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Provide Method to use i.e. SET,GET,REMOVE
    method: Method,
    /// Provide Key of type String
    #[arg(required = true)]
    key: String,
    /// Provide Value of type String
    value: Option<String>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Method {
    /// Sets key and value in storage
    Set,
    ///Gets value for given key
    Get,
    ///Removes value for given key
    Rm,
}

fn main() {
    let args = Args::parse();

    match args.method {
        Method::Set => {
            eprintln!("unimplemented");
            exit(1);
        }
        Method::Get => {
            eprintln!("unimplemented");
            exit(1);
        }
        Method::Rm => {
            eprintln!("unimplemented");
            exit(1);
        }
    }

    // let mut map = KvStore::new();
    // map.set(String::from("Ronit"), String::from("29"));
    //
    // println!("Age is{:?}",map.get("Ronit".to_string()).unwrap());
}
