use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Detect project root by searching upwards
fn find_project_root() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    let mut git_fallback: Option<PathBuf> = None;
    loop {
        if dir.join("requirements.txt").exists()
            || dir.join("pyproject.toml").exists()
        {
            return Ok(dir);
        }
        if dir.join(".git").exists() && git_fallback.is_none() {
            git_fallback = Some(dir.clone());
        }
        if !dir.pop() {
            break;
        }
    }

    if let Some(git_root) = git_fallback {
        return Ok(git_root);
    }

    Ok(env::current_dir()?)
}

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
    println!("{}", "Detecting project root...".cyan());

    let root = find_project_root()?;
    let output_path = root.join(output);

    println!(
        "{} {}",
        "Using project root:".cyan(),
        root.display()
    );

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

    fs::write(&output_path, content)
        .with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!(
        "{} {}",
        "Requirements written to".green(),
        output_path.display()
    );

    Ok(())
}

// Install dependencies and optionally regenerate requirements
pub fn sync(input: &str, all: bool) -> Result<()> {
    println!("{}", "Detecting project root...".cyan());

    let root = find_project_root()?;
    let input_path = root.join(input);

    println!(
        "{} {}",
        "Using project root:".cyan(),
        root.display()
    );

    let python = get_python_path()?;

    if !input_path.exists() {
        anyhow::bail!("{} not found", input_path.display());
    }

    let content = fs::read_to_string(&input_path)?;
    let needs_regen = is_unpinned(&content);

    println!("{}", "Installing dependencies...".cyan());

    let status = Command::new(&python)
        .args(["-m", "pip", "install", "-r", input])
        .current_dir(&root)
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
            println!("{}", "Using full dependency list".cyan());
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

        fs::write(&input_path, new_content)?;

        println!("{}", "requirements.txt updated with pinned versions".green());
    }

    Ok(())
}

// Detect if requirements file has unpinned dependencies
fn is_unpinned(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#') && !line.contains("==")
    })
}