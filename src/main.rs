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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .context("Failed to get home directory")?;
        
    let global_root = PathBuf::from(home_dir).join(".orbitsmith");
    
    if !global_root.join(KIT_SKILLS_PATH).exists() {
        println!("🚀 OrbitSmith kit not found locally. Cloning global toolkit to {:?}...", global_root);
        execute_command(Command::new("git").args([
            "clone",
            "https://github.com/zhugez/orbitsmith.git",
            &global_root.to_string_lossy()
        ]))?;
    }
    
    Ok(global_root)
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
    Command::new(tool)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
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
    if Path::new(SKILLS_DIR).exists() {
        fs::remove_dir_all(SKILLS_DIR).context("Failed to clean older skills directory")?;
    }
    
    let src_path = root.join(KIT_SKILLS_PATH);
    copy_dir_all(&src_path, SKILLS_DIR)
        .with_context(|| format!("Failed to copy skills from {:?}", src_path))?;
        
    println!("✅ Skills synced recursively without external dependencies");
    Ok(())
}

fn handle_doctor(root: &Path) -> Result<()> {
    println!("OrbitSmith Doctor");
    
    let tool = "git";
    let status = if is_tool_installed(tool) { "OK" } else { "MISSING" };
    println!("- {}: {}", tool, status);

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
