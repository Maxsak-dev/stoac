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
  .args(&["store", "load"])
  .required(true)
))]
#[command(group(
  ArgGroup::new("index_or_text")
    .args(&["index_store", "text_store"])
    .multiple(false)
    .requires("store") // this does not do anything for some reason
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
    value_name="INDEX",
    help="Will store the command from the history with a given index (0 -> last command, 1 -> command before last command)"
  )]
  index_store: Option<u32>,

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


}


fn store_last_command(tag: &str, command: &str) {
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

