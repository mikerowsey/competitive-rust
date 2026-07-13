use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct Category {
    category: String,
    problems: Vec<Problem>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Problem {
    id: u32,
    name: String,
    slug: String,
}

impl Problem {
    fn is_complete(&self) -> bool {
        Path::new(&format!("src/bin/{}.rs", self.slug)).exists()
    }
}

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn load_catalog() -> Vec<Category> {
    let json = std::fs::read_to_string("problems.json").expect("problems.json not found");
    serde_json::from_str(&json).expect("invalid problems.json")
}

fn print_usage(catalog: &[Category]) {
    let total: usize = catalog.iter().map(|c| c.problems.len()).sum();
    let done: usize = catalog
        .iter()
        .flat_map(|c| &c.problems)
        .filter(|p| p.is_complete())
        .count();

    println!("\n{BOLD}CSES Rust Solutions{RESET}\n===================");
    println!("  cargo run --bin <name>              Run a solution");
    println!("  cargo run --bin <name> < <path>     Run with stdin from a file");
    println!("  cargo test --bin <name>             Test a single solution");
    println!("  cargo test                          Test all solutions");
    println!();
    println!("  {done}/{total} problems completed");
    println!();
    println!("  cargo run -- --list                 List completed solutions");
    println!("  cargo run -- --categories           Completed, grouped by category");
    println!("  cargo run -- --all                  All problems (green = done, red = todo)");
}

fn print_list(catalog: &[Category]) {
    let done: Vec<&Problem> = catalog
        .iter()
        .flat_map(|c| &c.problems)
        .filter(|p| p.is_complete())
        .collect();

    println!("\n{BOLD}Completed Solutions ({}){RESET}\n", done.len());
    print_columns(done.iter().map(|p| p.slug.as_str()).collect());
}

fn print_categories(catalog: &[Category]) {
    let total_done: usize = catalog
        .iter()
        .flat_map(|c| &c.problems)
        .filter(|p| p.is_complete())
        .count();

    println!("\n{BOLD}Completed Solutions ({total_done}){RESET}\n");
    for cat in catalog {
        let done: Vec<&Problem> = cat.problems.iter().filter(|p| p.is_complete()).collect();
        if done.is_empty() {
            continue;
        }
        println!(
            "{BOLD}{}:{RESET} ({}/{})",
            cat.category,
            done.len(),
            cat.problems.len()
        );
        print_columns(done.iter().map(|p| p.slug.as_str()).collect());
        println!();
    }
}

fn print_all(catalog: &[Category]) {
    let total: usize = catalog.iter().map(|c| c.problems.len()).sum();
    let done: usize = catalog
        .iter()
        .flat_map(|c| &c.problems)
        .filter(|p| p.is_complete())
        .count();

    println!("\n{BOLD}All CSES Problems — {done}/{total} completed{RESET}\n");
    for cat in catalog {
        let cat_done = cat.problems.iter().filter(|p| p.is_complete()).count();
        println!(
            "{BOLD}{}:{RESET} ({}/{})",
            cat.category,
            cat_done,
            cat.problems.len()
        );
        print_colored_columns(&cat.problems);
        println!();
    }
}

fn print_columns(items: Vec<&str>) {
    let col_width = items.iter().map(|s| s.len()).max().unwrap_or(0) + 2;
    let term_width = 100;
    let cols = (term_width / col_width).max(1);

    for (i, item) in items.iter().enumerate() {
        if i % cols == 0 {
            print!("  ");
        }
        print!("{:<width$}", item, width = col_width);
        if (i + 1) % cols == 0 || i + 1 == items.len() {
            println!();
        }
    }
}

fn print_colored_columns(problems: &[Problem]) {
    let col_width = problems.iter().map(|p| p.slug.len()).max().unwrap_or(0) + 2;
    let term_width = 100;
    let cols = (term_width / col_width).max(1);

    for (i, p) in problems.iter().enumerate() {
        if i % cols == 0 {
            print!("  ");
        }
        let color = if p.is_complete() { GREEN } else { RED };
        print!("{color}{:<width$}{RESET}", p.slug, width = col_width);
        if (i + 1) % cols == 0 || i + 1 == problems.len() {
            println!();
        }
    }
}

fn main() {
    let catalog = load_catalog();
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("--list") => print_list(&catalog),
        Some("--categories") => print_categories(&catalog),
        Some("--all") => print_all(&catalog),
        _ => print_usage(&catalog),
    }
    println!();
}
