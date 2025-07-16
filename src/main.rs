// The elements data is taken from:
// https://github.com/Bowserinator/Periodic-Table-JSON

mod macros;
mod structs;

use crate::structs::Element;

use std::{
  cmp::min,
  env,
  fmt::Display,
  io::{self, Write},
  process,
};

use termion::{color, style};
use strsim::levenshtein as leven;
use inflector::{cases::sentencecase::to_sentence_case, cases::titlecase::to_title_case};

const MAX_WIDTH: usize = 80;

// Program starts here
fn main() {
  let (filter_opt, use_colors) = check_arguments();

  let filter = match filter_opt {
    Some(s) => s.to_lowercase(),
    None => ask_filter().to_lowercase(),
  };

  if filter.is_empty() {
    exit()
  }

  match find_element(get_elements(), filter) {
    Some(el) => {
      show_info(el, use_colors);
    }
    None => exit(),
  }
}

// Check if arguments were given and process flags
fn check_arguments() -> (Option<String>, bool) {
  let args: Vec<String> = env::args().collect();
  let mut filter = None;
  let mut use_colors = true;

  for arg in args.iter().skip(1) {
    if arg == "--no-colors" {
      use_colors = false;
    } else if filter.is_none() {
      filter = Some(s!(arg));
    }
  }

  (filter, use_colors)
}

// Centralized function to exit the program
fn exit() -> ! {
  process::exit(0)
}

// Centralized function to handle user input
pub fn get_input(message: &str) -> String {
  pp!(format!("{}: ", message));
  io::stdout().flush().unwrap();

  let mut input = s!();

  match io::stdin().read_line(&mut input) {
    Ok(_) => input,
    Err(_) => s!(),
  }
}

// If no argument was supplied
// then ask for a name to filter
fn ask_filter() -> String {
  get_input("Name, Symbol, or Number").trim().to_string()
}

// Reads and parses the JSON file
fn get_elements() -> Vec<Element> {
  let file = include_str!("elements.json");
  let obj = json::parse(&file).unwrap();
  let els = obj["elements"].dump();
  let elements: Vec<Element> = serde_json::from_str(&els).unwrap();
  elements
}

// Tries to find a specific element
fn find_element(els: Vec<Element>, filter: String) -> Option<Element> {
  let filtro = filter.trim().to_lowercase();
  let num = filtro.parse::<u32>().unwrap_or(0);
  let mut min_lev = 1000;
  let mut min_sim = Element::default();

  for el in els.iter() {
    if num > 0 {
      // If number then search
      // for the exact number
      if let Some(n) = el.number {
        if n == num {
          return Some(el.clone());
        }
      }
    } else {
      // Search for symbol
      if let Some(n) = &el.symbol {
        let s = n.to_lowercase();
        if s == filter {
          return Some(el.clone());
        }
      }

      // Search for name
      if let Some(n) = &el.name {
        let s = n.to_lowercase();
        if s == filter {
          return Some(el.clone());
        } else {
          // Check for similarity
          // for later use if no match yet
          let lev = leven(&s, &filtro);

          if lev < min_lev {
            min_lev = lev;
            min_sim = el.clone();
          }
        }
      }
    }
  }

  if num == 0 {
    // If not a number and there's
    // no match then find
    // the closest name
    if min_lev <= 3 {
      return Some(min_sim);
    }
  }

  return None;
}

// Generic function to print a property
fn print(s: &str, v: Option<impl Display>, case: &str, use_colors: bool) {
  if let Some(x) = v {
    let mut space = s!();

    for _ in 0..(s.len() + 2) {
      space += " ";
    }

    let mut sx = s!(x);
    if case == "title" {
      sx = to_title_case(&sx)
    } else if case == "sentence" {
      sx = to_sentence_case(&sx)
    }

    let txt = textwrap::fill(&sx, MAX_WIDTH);
    let text = s!(textwrap::indent(&txt, &space).trim());

    if use_colors {
      p!(format!(
        "{}{}{}: {}",
        color::Fg(color::Blue),
        s,
        color::Fg(color::Reset),
        text
      ))
    } else {
      p!(format!("{}: {}", s, text))
    }
  }
}

// Generic function to print list properties
fn print_list(s: &str, v: Option<Vec<impl Display>>, use_colors: bool) {
  if let Some(x) = v {
    if use_colors {
      pp!(format!(
        "{}{}{}: ",
        color::Fg(color::Blue),
        s,
        color::Fg(color::Reset)
      ));
    } else {
      pp!(format!("{}: ", s));
    }

    let sx = x.iter().map(|y| s!(y)).collect::<Vec<String>>().join(", ");

    let mut space = s!();

    for _ in 0..(s.len() + 2) {
      space += " ";
    }

    let txt = textwrap::fill(&sx, MAX_WIDTH);
    let text = s!(textwrap::indent(&txt, &space).trim());

    p!("{}", text);
  }
}

// Displays an element's properties
fn show_info(el: Element, use_colors: bool) {
  if use_colors {
    p!(format!(
      "\n{}{}{} ({}){}{}\n",
      style::Bold,
      color::Fg(color::Cyan),
      el.name.unwrap(),
      el.symbol.unwrap(),
      color::Fg(color::Reset),
      style::Reset
    ));
  } else {
    p!(format!(
      "\n{} ({})\n",
      el.name.unwrap(),
      el.symbol.unwrap()
    ));
  }

  print("Atomic Number", el.number, "", use_colors);
  print("Period Number", el.period, "", use_colors);
  print("Category", el.category, "title", use_colors);
  print("Summary", el.summary, "", use_colors);
  print("Discovered By", el.discovered_by, "", use_colors);
  print("Named By", el.named_by, "", use_colors);
  print("Appearance", el.appearance, "sentence", use_colors);
  print("Atomic Mass", el.atomic_mass, "", use_colors);
  print("Phase", el.phase, "", use_colors);
  print("Density", el.density, "", use_colors);
  print("Color", el.color, "title", use_colors);
  print("Molar Heat", el.molar_heat, "", use_colors);
  print("Melting Point", el.melt, "", use_colors);
  print("Boiling Point", el.boil, "", use_colors);
  print_list("Shells", el.shells, use_colors);
  print("Electron Configuration", el.electron_configuration, "", use_colors);
  print("Electron Affinity", el.electron_affinity, "", use_colors);
  print(
    "Electronegativity Pauling",
    el.electronegativity_pauling,
    "",
    use_colors,
  );
  print_list("Ionization Energies", el.ionization_energies, use_colors);
  print("X Pos", el.xpos, "", use_colors);
  print("Y Pos", el.ypos, "", use_colors);
  print("Source", el.source, "", use_colors);
  print("Spectral Image", el.spectral_img, "", use_colors);

  p!("");
}
