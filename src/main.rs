use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::env;
use dirs;
use clap::{ArgGroup, Parser};
use rustyline::DefaultEditor;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(
  version,
  about = "stoac - store a command, a helper to keep your cli organized",
  long_about = None
)]
#[command(group(
  ArgGroup::new("required")
  .args(&["store", "load", "print", "delete"])
  .required(true)
))]
#[command(group(
  ArgGroup::new("store_group")
    .args(&["text_store", "index_store", "interactive_store"])
    .multiple(false)
    .requires("store") // this does not do anything for some reason
))]
#[command(group(
  ArgGroup::new("print_group")
    .args(&["print_output"])
    .multiple(false)
    .requires("load") // this does not do anything for some reason
))]
struct Args {
  #[arg(
    short,
    long,
    value_name="TAG",
    help="Loads a command at a given tag if it exists"
  )]
  load: Option<String>,

  #[arg(
    short,
    long,
    value_name="TAG",
    help="Stores a command to a given tag"
  )]
  store: Option<String>,

  #[arg(
    short,
    long,
    value_name="TAG",
    help="Deletes a specified command by tag if it exists"
  )]
  delete: Option<String>,

  #[arg(
    long="print",
    help="Prints the content of the database into the shell"
  )]
  print: bool,

  #[arg(
    short,
    long,
    value_name="COMMAND",
    help="Will store a custom command from text. Make sure to encapsulate it in quotes and escape necessary quotes"
  )]
  text_store: Option<String>,

  #[arg(
    short,
    long,
    help="Will print the command to execute to stdout instead of using the interactive mode (useful for shell integration)"
  )]
  print_output: bool,

  #[arg(
    short,
    long,
    help="Will store a custom command from from an interactive text field"
  )]
  interactive_store: bool,

  #[arg(
    short='x',
    long,
    value_name="INDEX",
    help="Will store a command from the history of your shell at the specified index"
  )]
  index_store: Option<usize>,

  #[arg(
    long,
    value_name="bash | zsh",
    help="Overrides the shells history to be used (only applicable when storing by index)"
  )]
  shell: Option<String>,
}


fn main() {
  let args = Args::parse();

  if args.print {
    print_db();
    return;
  }

  if args.load.is_some() {
    if args.text_store.is_some() || args.index_store.is_some() || args.shell.is_some() || args.interactive_store {
      eprintln!("[WARNING]: Additional arguments are ignored when loading a command");
    }
    if args.print_output {
      print_command(&args.load.unwrap(), true);
    }
    else {
      print_command(&args.load.unwrap(), false);
    }
    return;
  }

  if args.delete.is_some() {
    let tag = args.delete.unwrap();
    delete_command(&tag);
    return;
  }

  if args.store.is_some() {
    if args.interactive_store {
      store_command(&args.store.unwrap(), "");
    } else if args.text_store.is_some() {
      store_command(&args.store.unwrap(), &args.text_store.unwrap());
    } else if args.index_store.is_some() {
      let shell_hint = args.shell.unwrap_or("".to_string());
      let command = get_command_from_history(args.index_store.unwrap(), shell_hint);
      store_command(&args.store.unwrap(), &command);
    } else {
      eprintln!("Specified nothing to store");
      std::process::exit(-1);
    }
  }
}


fn store_command(tag: &str, command: &str) {
  println!("Storing command for '{}' (Press enter to store or Ctrl+C to abort)", tag);
  let edited_command = user_edit_mode(command);

  let db: sled::Db = sled::open(get_db_path()).unwrap();

  db.insert(tag, edited_command.as_bytes()).unwrap();
  db.flush().unwrap();
}


fn delete_command(tag: &str) {
  let db: sled::Db = sled::open(get_db_path()).unwrap_or_else(|_| {
    eprint!("Error opening the database. Make sure it exists by storing at least one command");
    std::process::exit(-1);
  });

  if let Err(_) = db.remove(tag) {
    println!("Could not delete command for {}", tag);
  } else {
    println!("Successfully deleted command for {}", tag);
  }
}


fn print_command(tag: &str, print_only: bool) {
  let db: sled::Db = sled::open(get_db_path()).unwrap_or_else(|_| {
    eprint!("Error opening the database. Make sure it exists by storing at least one command");
    std::process::exit(-1);
  });

  let exact_result = db.get(tag).unwrap_or_else(|e| {
    eprint!("Error communicating with the database ({})", e);
    std::process::exit(-1);
  });

  if let Some(exact_val) = exact_result {
    let command = String::from_utf8(exact_val.to_vec()).unwrap();
    if print_only {
      println!("{}", command);
    }
    else {
      println!("Command for '{}' (Press enter to execute or Ctrl+C to abort)", tag);
      execute_command(&command);
    }
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
      eprintln!("No valid commands found for {}", tag);
    }
    Err(e) => {
      eprintln!("Error while fetching entries from db: {}", e);
    }
  }

  std::process::exit(-1);
}


fn get_db_path() -> PathBuf {
  let path_end = Path::new("stoac/db");


  if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
    return PathBuf::from(xdg_config_home).join(path_end);
  }

  if let Ok(home) = env::var("HOME") {
    let config_path = PathBuf::from(home).join(".config").join(path_end);
    return config_path;
  }

  println!("Could not open config paths (Check that either XDG_CONFIG_HOME or HOME environment variable is set");
  std::process::exit(-1);
}


fn user_edit_mode(initial_command: &str) -> String {
  let mut rl = DefaultEditor::new().unwrap();
  let input = rl.readline_with_initial("", (initial_command, "")).unwrap_or_else(|_| {
    println!("Aborted.");
    std::process::exit(0);
  });

  return input;
}


fn execute_command(initial_command: &str) {
  let edited_command = user_edit_mode(initial_command);

  println!("-----");

  let status = Command::new("sh")
    .arg("-c")
    .arg(edited_command)
    .status()
    .expect("Failed to spawn command");

  std::process::exit(status.code().unwrap_or(1));   
}


fn print_db() {
  let db: sled::Db = sled::open(get_db_path()).unwrap_or_else(|_| {
    eprint!("Error opening the database. Make sure it exists by storing at least one command");
    std::process::exit(-1);
  });

  for entry in db.iter() {
    let (key, value) = entry.unwrap();
    println!("Tag: {}, Command: {}", String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
  }
}


fn get_command_from_history(line_num: usize, shell_hint: String) -> String {
  if shell_hint.to_ascii_lowercase() == "bash" {
    return get_bash_command(line_num);
  } else if shell_hint.to_ascii_lowercase() == "zsh" {
    return get_zsh_command(line_num);
  } else if shell_hint == "" {
  } else {
    println!("Provided unsupported shell hint flag ({}). Aborting.", shell_hint);
    std::process::exit(1);
  }

  let shell_var = env::var("SHELL");
  if shell_var.is_err() {
    println!("SHELL environment variable not set. Please set it or specify your shell using the flag");
    std::process::exit(1);
  }

  let shell_str = shell_var.unwrap();

  let zsh_pattern = r"zsh";
  let zsh_re = Regex::new(zsh_pattern).unwrap();

  if zsh_re.is_match(&shell_str) {
    return get_zsh_command(line_num);
  }

  let bash_pattern = r"bash";
  let bash_re = Regex::new(bash_pattern).unwrap();

  if bash_re.is_match(&shell_str) {
    return get_bash_command(line_num);
  }

  println!("The program was not started from Bash or Zsh. Other shells are currently not supported");
  std::process::exit(1);
}


fn get_line_from_file(line_num: usize, file_path: &str) -> String {
  let home_dir = dirs::home_dir().expect("Could not find home directory");

  let mut history_path = PathBuf::from(home_dir);
  history_path.push(file_path);

  let file = File::open(history_path).unwrap_or_else(|_| {
    println!("Could not open .bash_history");
    std::process::exit(1);
  });

  let reader = io::BufReader::new(file);

  for (index, line) in reader.lines().enumerate() {
    if index + 1 != line_num { continue; }

    if index + 1 == line_num {
      let str_line = line.expect("Could not read line");
      return str_line;
    }
  }

  println!("Could not find history index {}", line_num);
  std::process::exit(1);
}


fn get_zsh_command(line_num: usize) -> String {
  let history_line = get_line_from_file(line_num, ".zsh_history");

  if let Some(pos) = history_line.find(";") {
    return history_line[pos + 1..].to_string();
  } else {
    return "".to_string();
  }
}


fn get_bash_command(line_num: usize) -> String {
  return get_line_from_file(line_num, ".bash_history");
}
