use std::process::Command;
use clap::{ArgGroup, Parser};

static DB_PATH: &str = "stoac-db";

#[derive(Parser, Debug)]
#[command(
  version,
  about = "stoac - store a command, a helper to keep your cli organized",
  long_about = None
)]
#[command(group(
  ArgGroup::new("required")
  .args(&["store", "load", "print"])
  .required(true)
))]
#[command(group(
  ArgGroup::new("text_group")
    .args(&["text_store"])
    .multiple(false)
    .requires("store") // this does not do anything for some reason
))]
#[command(group(
  ArgGroup::new("print_group")
    .args(&["print"])
    .multiple(false)
    .requires("load") // this does not do anything for some reason
))]
struct Args {
  #[arg(
    short,
    long,
    value_name="TAG",
    help="Loads a command at a given tag if specified"
  )]
  load: Option<String>,

  #[arg(
    short,
    long,
    value_name="TAG",
    help="Stores a command to a given tag if specified"
  )]
  store: Option<String>,

  #[arg(
    short,
    long,
    help="Prints the content of the database into the shell"
  )]
  print: bool,

  #[arg(
    short,
    long,
    value_name="COMMAND",
    help="Will store a custom command from text. Make sure to encapsulate it in quotes"
  )]
  text_store: Option<String>,
}


fn main() {
  let args = Args::parse();

  if args.print {
    print_db();
    return;
  }

  // loading flow
  if args.load.is_some() {
    if args.text_store.is_some() {
      eprintln!("[WARNING]: Additional arguments are ignored when loading a command");
    }
  }
}


fn store_command(tag: &str, command: &str) {
  let db: sled::Db = sled::open(DB_PATH).unwrap();

  db.insert(tag, command).unwrap();
  db.flush().unwrap();
}


fn print_db() {
  let db: sled::Db = sled::open(DB_PATH).unwrap();

  for entry in db.iter() {
    let (key, value) = entry.unwrap();
    println!("Key: {}, Value: {}", String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
  }
}

