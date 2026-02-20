use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const AGENT_DIR: &str = ".agent";
const SKILLS_DIR: &str = "skills";
const DOCS_DIR: &str = "docs";
const KIT_SKILLS_PATH: &str = "kit/skills";
const AGENTS_MD_SRC: &str = "kit/AGENTS.md";
const AGENTS_MD_TARGET: &str = ".agent/AGENTS.md";

#[derive(Parser)]
#[command(name = "orbitsmith", version, about = "OrbitSmith CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    SyncSkills,
    Doctor,
    Update,
}

fn repo_root() -> Result<PathBuf> {
    let exe = env::current_exe()?;
    let bin_dir = exe
        .parent()
        .context("Failed to get executable parent directory")?;
    let root = bin_dir
        .parent()
        .context("Failed to get repository root directory")?;
    Ok(root.to_path_buf())
}

fn execute_command(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute command: {:?}", cmd))?;
    if !status.success() {
        anyhow::bail!("Command failed with status {:?}: {:?}", status, cmd);
    }
    Ok(())
}

fn is_tool_installed(tool: &str) -> bool {
    Command::new("sh")
        .args(["-lc", &format!("command -v {} >/dev/null 2>&1", tool)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn handle_init(root: &Path) -> Result<()> {
    fs::create_dir_all(AGENT_DIR).with_context(|| format!("Failed to create {}", AGENT_DIR))?;
    fs::create_dir_all(SKILLS_DIR).with_context(|| format!("Failed to create {}", SKILLS_DIR))?;
    fs::create_dir_all(DOCS_DIR).with_context(|| format!("Failed to create {}", DOCS_DIR))?;

    let target = PathBuf::from(AGENTS_MD_TARGET);
    if !target.exists() {
        fs::copy(root.join(AGENTS_MD_SRC), &target)
            .with_context(|| format!("Failed to copy {} to {}", AGENTS_MD_SRC, AGENTS_MD_TARGET))?;
    }

    println!("✅ Workspace initialized for OrbitSmith");
    Ok(())
}

fn handle_sync_skills(root: &Path) -> Result<()> {
    fs::create_dir_all(SKILLS_DIR).with_context(|| format!("Failed to create {}", SKILLS_DIR))?;
    execute_command(Command::new("rsync").args([
        "-a",
        "--delete",
        root.join(KIT_SKILLS_PATH).to_string_lossy().as_ref(), // Adding trailing slash like original can be tricky with pathbuf, let's keep it safe.
        // Actually rsync behavior with trailing slash is important. Let's ensure it.
        &format!("{}/", root.join(KIT_SKILLS_PATH).to_string_lossy()),
        &format!("{}/", SKILLS_DIR),
    ]))?;
    println!("✅ Skills synced");
    Ok(())
}

fn handle_doctor(root: &Path) -> Result<()> {
    println!("OrbitSmith Doctor");

    for tool in ["git", "rsync"] {
        let status = if is_tool_installed(tool) {
            "OK"
        } else {
            "MISSING"
        };
        println!("- {}: {}", tool, status);
    }

    let kit_skills_status = if root.join(KIT_SKILLS_PATH).exists() {
        "OK"
    } else {
        "MISSING"
    };
    println!("- kit skills: {}", kit_skills_status);

    Ok(())
}

fn handle_update(root: &Path) -> Result<()> {
    execute_command(
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["pull", "--ff-only"]),
    )?;
    println!("✅ OrbitSmith updated");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = repo_root()?;

    match cli.command {
        Commands::Init => handle_init(&root)?,
        Commands::SyncSkills => handle_sync_skills(&root)?,
        Commands::Doctor => handle_doctor(&root)?,
        Commands::Update => handle_update(&root)?,
    }

    Ok(())
}
