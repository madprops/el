// The elements data is taken from:
// https://github.com/Bowserinator/Periodic-Table-JSON

mod macros;
mod structs;

use crate::
{
    structs::
    {
        Element,
    }
};

use std::
{
    process, env,
    io::{self, Write},
    fmt::Display,
    cmp::min,
};

use termion::
{
    color, style,
};

use strsim::
{
    levenshtein as leven,
};

use inflector::
{
    cases::titlecase::to_title_case,
    cases::sentencecase::to_sentence_case,
};

const MAX_WIDTH: usize = 80;

// Program starts here
fn main() 
{
    let filter = match check_arguments()
    {
        Some(s) => s,
        None => 
        {
            ask_filter()
        }
    };

    if filter.is_empty() {exit()}
    
    match find_element(get_elements(), filter)
    {
        Some(el) =>
        {
            show_info(el);
        },
        None => exit()
    }
}

// Centralized function to exit the program
fn exit() -> !
{
    process::exit(0)
}

// Check if arguments were given
fn check_arguments() -> Option<String>
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {return None}
    Some(s!(args[1]))
}

// Centralized function to handle user input
pub fn get_input(message: &str) -> String
{
    pp!(format!("{}: ", message));
    io::stdout().flush().unwrap();

    let mut input = s!();

    match io::stdin().read_line(&mut input) 
    {
        Ok(_) => input,
        Err(_) => s!()
    }
}

// If no argument was supplied
// then ask for a name to filter
fn ask_filter() -> String
{
    get_input("Name or Number")
}

// Reads and parses the JSON file
fn get_elements() -> Vec<Element>
{
    let file = include_str!("elements.json");
    let obj = json::parse(&file).unwrap(); let els = obj["elements"].dump();
    let elements: Vec<Element> = serde_json::from_str(&els).unwrap();
    elements
}

// Tries to find a specific element
fn find_element(els: Vec<Element>, filter: String) -> Option<Element>
{
    let filtro = filter.trim().to_lowercase();
    let num = filtro.parse::<u32>().unwrap_or(0);
    let mut min_lev = 1000;
    let mut min_sim = Element::default();

    for el in els.iter()
    {
        if num > 0
        {
            // If number then search
            // for the exact number
            if let Some(n) = el.number
            {
                if n == num {return Some(el.clone())}
            }
        }

        else
        {
            // If text then search
            // for the exact name
            if let Some(n) = &el.name
            {   
                let s = n.to_lowercase();

                if s == filtro
                {
                    return Some(el.clone());
                }

                else
                {
                    // Check for similarity 
                    // for later use
                    let lev = leven(&s , &filtro);

                    if lev < min_lev
                    {
                        min_lev = lev;
                        min_sim = el.clone();
                    }
                }
            }
        }
    }

    if num == 0
    {
        // If not a number and there's
        // no match then find
        // the closest name
        if min_lev <= 3
        {
            return Some(min_sim);
        }
    }

    return None
}

// Generic function to print a property
fn print(s: &str, v: Option<impl Display>, case: &str)
{
    if let Some(x) = v
    {
        let mut space = s!();

        for _ in 0..(s.len() + 2)
        {
            space += " ";
        }

        let mut sx = s!(x);
        if case == "title" {sx = to_title_case(&sx)}
        else if case == "sentence" {sx = to_sentence_case(&sx)}
        let n = termion::terminal_size().unwrap().0 as usize - s.len() - 5;
        let txt = textwrap::fill(&sx, min(MAX_WIDTH, n)); let text = s!(textwrap::indent(&txt, &space).trim());
        p!(format!("{}{}{}: {}", color::Fg(color::Blue), s, color::Fg(color::Reset), text))
    }
}

// Generic function to print list properties
fn print_list(s: &str, v: Option<Vec<impl Display>>)
{
    if let Some(x) = v
    {
        pp!(format!("{}{}{}: ", color::Fg(color::Blue), s, color::Fg(color::Reset)));

        let sx = x.iter().map(|y| s!(y))
            .collect::<Vec<String>>().join(", ");
        
        let mut space = s!();

        for _ in 0..(s.len() + 2)
        {
            space += " ";
        }
        
        let n = termion::terminal_size().unwrap().0 as usize - s.len() - 5;
        let txt = textwrap::fill(&sx, min(MAX_WIDTH, n)); let text = s!(textwrap::indent(&txt, &space).trim());
        
        p!("{}", text);
    }
}

// Displays an element's properties
fn show_info(el: Element)
{
    p!(format!("\n{}{}{} ({}){}{}\n",
        style::Bold,
        color::Fg(color::Cyan), 
        el.name.unwrap(), 
        el.symbol.unwrap(), 
        color::Fg(color::Reset),
        style::Reset));

    print("Atomic Number", el.number, "");
    print("Period Number", el.period, "");
    print("Category", el.category, "title");
    print("Summary", el.summary, "");
    print("Discovered By", el.discovered_by, "");
    print("Named By", el.named_by, "");
    print("Appearance", el.appearance, "sentence");
    print("Atomic Mass", el.atomic_mass, "");
    print("Phase", el.phase, "");
    print("Density", el.density, "");
    print("Color", el.color, "title");
    print("Molar Heat", el.molar_heat, "");
    print("Melting Point", el.melt, "");
    print("Boiling Point", el.boil, "");
    print_list("Shells", el.shells);
    print("Electron Configuration", el.electron_configuration, "");
    print("Electron Affinity", el.electron_affinity, "");
    print("Electronegativity Pauling", el.electronegativity_pauling, "");
    print_list("Ionization Energies", el.ionization_energies);
    print("X Pos", el.xpos, "");
    print("Y Pos", el.ypos, "");
    print("Source", el.source, "");
    print("Spectral Image", el.spectral_img, "");

    p!("");
}