use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Get Python path from active virtual environment
fn get_python_path() -> Result<String> {
    let venv = env::var("VIRTUAL_ENV")
        .context("No active virtual environment. Activate one first.")?;

    let python_path = if cfg!(windows) {
        format!(r"{}\Scripts\python.exe", venv)
    } else {
        format!("{}/bin/python", venv)
    };

    if !Path::new(&python_path).exists() {
        anyhow::bail!("Python not found in virtual environment");
    }

    Ok(python_path)
}

// Generate requirements file
pub fn generate(output: &str, all: bool) -> Result<()> {
    println!("{}", "Detecting virtual environment...".cyan());

    let python = get_python_path()?;

    let args = if all {
        println!("{}", "Using full dependency list".cyan());
        vec!["-m", "pip", "freeze"]
    } else {
        println!("{}", "Using direct dependencies only".cyan());
        println!("{}", "Tip: use --all to include full dependency tree".dimmed());
        vec!["-m", "pip", "list", "--not-required", "--format=freeze"]
    };

    let result = Command::new(&python)
        .args(args)
        .output()
        .context("Failed to run pip command")?;

    if !result.status.success() {
        anyhow::bail!("pip command failed");
    }

    let content = String::from_utf8(result.stdout)?;

    fs::write(output, content)
        .with_context(|| format!("Failed to write {}", output))?;

    println!(
        "{} {}",
        "Requirements written to".green(),
        output.bold()
    );

    Ok(())
}

// Install dependencies from requirements file
pub fn sync(input: &str, all: bool) -> Result<()> {
    println!("{}", "Detecting virtual environment...".cyan());

    let python = get_python_path()?;

    if !Path::new(input).exists() {
        anyhow::bail!("{} not found", input);
    }

    let content = fs::read_to_string(input)?;
    let needs_regen = is_unpinned(&content);

    println!("{}", "Installing dependencies...".cyan());

    let status = Command::new(&python)
        .args(["-m", "pip", "install", "-r", input])
        .status()
        .context("Failed to run pip install")?;

    if !status.success() {
        anyhow::bail!("Installation failed");
    }

    println!("{}", "Dependencies installed".green());

    // Auto regenerate if unpinned
    if needs_regen {
        println!("{}", "Detected unpinned dependencies, regenerating...".yellow());

        let args = if all {
            println!("{}", "Using full dependency list (pip freeze)".cyan());
            vec!["-m", "pip", "freeze"]
        } else {
            println!("{}", "Using direct dependencies only".cyan());
            vec!["-m", "pip", "list", "--not-required", "--format=freeze"]
        };

        let result = Command::new(&python)
            .args(args)
            .output()
            .context("Failed to regenerate requirements")?;

        if !result.status.success() {
            anyhow::bail!("Failed to regenerate requirements");
        }

        let new_content = String::from_utf8(result.stdout)?;

        fs::write(input, new_content)?;

        println!("{}", "requirements.txt updated with pinned versions".green());
    }

    Ok(())
}

// To check if library has a version number
fn is_unpinned(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#') && !line.contains("==")
    })
}