use tree_sitter::{Parser, Node};
use walkdir::WalkDir;
use std::fs;

pub fn extract_function_names_from_dir(dir: &str) -> Vec<String> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");

    let mut functions = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(source) = fs::read_to_string(entry.path()) {
            if let Some(tree) = parser.parse(&source, None) {
                let root_node = tree.root_node();
                visit_node(&source, root_node, &mut functions, None);
            }
        }
    }

    functions
}

fn visit_node(source: &str, node: Node, functions: &mut Vec<String>, current_impl: Option<&str>) {
    match node.kind() {
        "impl_item" => {
            // Find the struct/type name for this impl block
            if let Some(type_node) = node.child_by_field_name("type") {
                if let Ok(impl_name) = type_node.utf8_text(source.as_bytes()) {
                    for i in 0..node.child_count() {
                        visit_node(source, node.child(i).unwrap(), functions, Some(impl_name));
                    }
                }
            }
        }
        "function_item" => {
            if let Some(identifier) = node.child_by_field_name("name") {
                if let Ok(name) = identifier.utf8_text(source.as_bytes()) {
                    if let Some(impl_name) = current_impl {
                        functions.push(format!("{}::{}", impl_name, name));
                    } else {
                        functions.push(name.to_string());
                    }
                }
            }
            // Continue visiting children
            for i in 0..node.child_count() {
                visit_node(source, node.child(i).unwrap(), functions, current_impl);
            }
        }
        _ => {
            // Continue visiting other nodes
            for i in 0..node.child_count() {
                visit_node(source, node.child(i).unwrap(), functions, current_impl);
            }
        }
    }
}
