use std::process::{Command, exit};

use clap::{ArgGroup, Parser};
use rustyline::DefaultEditor;

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
    print_command(&args.load.unwrap());
  }

  if args.store.is_some() {
    if args.text_store.is_none() {
      eprintln!("No command to store specified");
      std::process::exit(-1);
    }
    store_command(&args.store.unwrap(), &args.text_store.unwrap());
  }
}


fn store_command(tag: &str, command: &str) {
  let db: sled::Db = sled::open(DB_PATH).unwrap();

  db.insert(tag, command).unwrap();
  db.flush().unwrap();
}


fn print_command(tag: &str) {
  let db: sled::Db = sled::open(DB_PATH).unwrap();

  let exact_result = db.get(tag).unwrap_or_else(|e| {
    eprint!("Error communicating with the database ({})", e);
    std::process::exit(-1);
  });

  if let Some(exact_val) = exact_result {
    let command = String::from_utf8(exact_val.to_vec()).unwrap();
    println!("Command for '{}' (Press enter to execute or Ctrl+C to abort)", tag);
    user_edit_mode(&command);
    return;
  }

  let tag_bytes = tag.as_bytes();
  let mut upper_bound = tag_bytes.to_vec();
  if let Some(last_byte) = upper_bound.last_mut() {
    *last_byte += 1;
  }
  let tag_bound_bytes = upper_bound.as_slice();

  let similar_results = db.range(tag_bytes..tag_bound_bytes).collect::<Result<Vec<_>, _>>();

  match similar_results {
    Ok(entries) if !entries.is_empty() => {
      eprintln!("No valid commands found, did you mean:");
      for (key, _) in entries {
        eprint!("{}\n", String::from_utf8(key.to_vec()).unwrap());
      }
    }
    Ok(_) => {
      eprintln!("No valid commands found");
    }
    Err(e) => {
      eprintln!("Error while fetching entries from db: {}", e);
    }
  }
}


fn user_edit_mode(initial_command: &str) {
  let mut rl = DefaultEditor::new().unwrap();
  let input = rl.readline_with_initial("", (initial_command, "")).unwrap_or_else(|_| {
    println!("Aborted.");
    std::process::exit(0);
  });

  println!("-----");

  let status = Command::new("sh")
    .arg("-c")
    .arg(input)
    .status()
    .expect("Failed to spawn command");

    exit(status.code().unwrap_or(1));   
}


fn print_db() {
  let db: sled::Db = sled::open(DB_PATH).unwrap();

  for entry in db.iter() {
    let (key, value) = entry.unwrap();
    println!("Key: {}, Value: {}", String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
  }
}

