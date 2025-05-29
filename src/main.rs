mod flow_reader;
mod mermaid;
mod parser;
mod utils;

use clap::{Arg, Command};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use inquire::{Select, Text};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("docuflows")
        .version("0.1.0")
        .about("Visualize feature flows in codebases with Docuflows")
        .subcommand(
            Command::new("diagram")
                .short_flag('d')
                .long_flag("diagram")
                .about("Generate a Mermaid.js diagram from a .docuflow file")
                .arg(
                    Arg::new("flow")
                        .help("Name of the .docuflow file (without extension)")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("parse")
                .short_flag('p')
                .long_flag("parse")
                .about("Parse source directory and list all function names")
                .arg(Arg::new("path").help("Source path").required(true)),
        )
        .subcommand(
            Command::new("trace")
                .short_flag('t')
                .long_flag("trace")
                .about("create flow")
                .arg(Arg::new("path").help("Source path").required(true)),
        )
        .get_matches();

    if let Some(diagram_matches) = matches.subcommand_matches("diagram") {
        let flow_name = diagram_matches.get_one::<String>("flow").unwrap();
        let flow_path = PathBuf::from(".docuflows").join(format!("{}.docuflow", flow_name));
        let flow = flow_reader::load_flow_file(&flow_path).expect("Failed to load flow file");
        let mermaid_output = mermaid::generate_mermaid_diagram(&flow);
        println!("{}", mermaid_output);
    } else if let Some(parse_matches) = matches.subcommand_matches("parse") {
        let path = parse_matches.get_one::<String>("path").unwrap();
        let functions = parser::extract_function_names_from_dir(path);
        for func in &functions {
            println!("{}", func);
        }
    } else if let Some(trace_matches) = matches.subcommand_matches("trace") {
        let path = trace_matches.get_one::<String>("path").unwrap();
        let functions = parser::extract_function_names_from_dir(&path);
        println!("Parsed {} functions from {}", &functions.len(), path);
        let search_term = Text::new("Search for a function to start").prompt();

        match search_term {
            Ok(search_term) => {
                if search_term.is_empty() {
                    return;
                }
                let matcher = SkimMatcherV2::default();
                let mut matches = Vec::new();

                for candidate in functions {
                    if let Some(_) = matcher.fuzzy_match(&candidate, &search_term) {
                        matches.push(candidate);
                    }
                }

                let selected_function = Select::new("Select a starting function", matches).prompt();

                match selected_function {
                    Ok(function_name) => {
                        let view_callers = &format!("View callers of {}", function_name);
                        let view_called = &format!("View methods called in {}", function_name);
                        let options: Vec<&str> =
                            vec![view_callers, view_called, "Go back", "Cancel"];
                        let next_step = Select::new(
                            &format!("What would you like to do with {}?", function_name),
                            options,
                        )
                        .prompt();

                        match next_step {
                            Ok(a) => println!("Next: {:?}", a),
                            Err(e) => utils::print_error(e),
                        }
                    }
                    Err(e) => utils::print_error(e),
                }
            }
            Err(e) => utils::print_error(e),
        }
    }
}