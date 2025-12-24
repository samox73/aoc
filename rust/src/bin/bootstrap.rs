use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "bootstrap")]
#[command(about = "Bootstrap a new Advent of Code day from template", long_about = None)]
struct Cli {
    /// Year (e.g., 2024)
    year: u16,
    /// Day (1-25)
    day: u8,
}

fn main() {
    let cli = Cli::parse();
    init_day(cli.year, cli.day);
}

fn init_day(year: u16, day: u8) {
    // Validate day
    if !(1..=25).contains(&day) {
        eprintln!("Error: Day must be between 1 and 25");
        process::exit(1);
    }

    let src_dir = get_src_dir();
    let year_dir = src_dir.join(format!("year{}", year));
    let day_dir = year_dir.join(format!("day{:02}", day));
    let template_dir = src_dir.join("template").join("dayXX");

    // Check if day already exists
    if day_dir.exists() {
        eprintln!("Error: year{}/day{:02} already exists!", year, day);
        process::exit(1);
    }

    // Check if template exists
    if !template_dir.exists() {
        eprintln!("Error: Template directory not found at {:?}", template_dir);
        process::exit(1);
    }

    // Create year directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&year_dir) {
        eprintln!("Error creating year directory: {}", e);
        process::exit(1);
    }

    // Copy template
    if let Err(e) = copy_dir_recursive(&template_dir, &day_dir) {
        eprintln!("Error copying template: {}", e);
        process::exit(1);
    }

    // Replace placeholders in the new day's mod.rs
    let day_mod_rs = day_dir.join("mod.rs");
    if let Err(e) = replace_placeholders(&day_mod_rs, year, day) {
        eprintln!("Error updating placeholders: {}", e);
        process::exit(1);
    }

    // Add module declaration to year's mod.rs
    let year_mod_rs = year_dir.join("mod.rs");
    let module_line = format!("pub mod day{:02};", day);

    if year_mod_rs.exists() {
        // Check if module already declared
        if let Ok(content) = fs::read_to_string(&year_mod_rs) {
            if !content.contains(&module_line) {
                if let Err(e) = fs::write(&year_mod_rs, format!("{}\n{}", content, module_line)) {
                    eprintln!("Error updating mod.rs: {}", e);
                    process::exit(1);
                }
            }
        }
    } else {
        // Create new mod.rs
        if let Err(e) = fs::write(&year_mod_rs, format!("{}\n", module_line)) {
            eprintln!("Error creating mod.rs: {}", e);
            process::exit(1);
        }
    }

    // Update run.rs to add the match arm
    let run_rs = src_dir.join("bin").join("run.rs");
    if let Err(e) = update_main_rs(&run_rs, year, day) {
        eprintln!("Warning: Could not update run.rs automatically: {}", e);
        eprintln!("Please manually add the following line to run_year_{} function:", year);
        eprintln!("    {} => year{}::day{:02}::solve(input),", day, year, day);
    } else {
        println!("✓ Updated run.rs dispatch");
    }

    println!("✓ Created year{}/day{:02}", year, day);
    println!("✓ Run 'cargo run --bin run {} {}' to work on this day", year, day);
}

fn get_src_dir() -> PathBuf {
    // Get the manifest directory (project root)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("src")
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn replace_placeholders(file_path: &Path, year: u16, day: u8) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let updated_content = content
        .replace("YYYY", &year.to_string())
        .replace("DD", &format!("{:02}", day));
    fs::write(file_path, updated_content)
}

fn update_main_rs(main_rs_path: &Path, year: u16, day: u8) -> std::io::Result<()> {
    let content = fs::read_to_string(main_rs_path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Find the function for this year
    let function_name = format!("fn run_year_{}(", year);
    let function_start = lines.iter().position(|line| line.contains(&function_name))
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not find {} function", function_name)
        ))?;

    // Find the default match arm (starting with "_")
    let mut default_arm_line = None;
    for (idx, line) in lines.iter().enumerate().skip(function_start) {
        let trimmed = line.trim();
        if trimmed.starts_with("_ =>") {
            default_arm_line = Some(idx);
            break;
        }
        // Stop if we hit the next function
        if idx > function_start && line.starts_with("fn ") {
            break;
        }
    }

    let insert_line = default_arm_line.ok_or_else(|| std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find default match arm"
    ))?;

    // Check if this day is already in the match
    let new_match_arm = format!("{} => year{}::day{:02}::solve(input),", day, year, day);
    if content.contains(&new_match_arm) {
        return Ok(()); // Already exists, nothing to do
    }

    // Build the new content
    let mut new_lines: Vec<String> = lines[..insert_line].to_vec();

    // Determine indentation from the default arm line
    let indent = lines[insert_line].chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    // Add the new match arm
    new_lines.push(format!("{}{}", indent, new_match_arm));
    new_lines.extend_from_slice(&lines[insert_line..]);

    let new_content = new_lines.join("\n") + "\n";
    fs::write(main_rs_path, new_content)?;

    Ok(())
}
