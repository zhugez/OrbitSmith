use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const AGENT_DIR: &str = ".agent";
const KIT_AGENT_PATH: &str = "kit/.agent";

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
    Status,
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

    if !global_root.join(KIT_AGENT_PATH).exists() {
        println!(
            "🚀 OrbitSmith kit not found locally. Cloning global toolkit to {:?}...",
            global_root
        );
        execute_command(Command::new("git").args([
            "clone",
            "https://github.com/zhugez/orbitsmith.git",
            &global_root.to_string_lossy(),
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
    if Path::new(AGENT_DIR).exists() {
        println!(
            "⚠️ Workspace already initialized (found {}). Cleaning up first...",
            AGENT_DIR
        );
        fs::remove_dir_all(AGENT_DIR).context("Failed to clean older .agent directory")?;
    }

    let src_path = root.join(KIT_AGENT_PATH);
    copy_dir_all(&src_path, AGENT_DIR)
        .with_context(|| format!("Failed to copy kit from {:?}", src_path))?;

    println!(
        "✅ Workspace initialized. Antigravity Kit placed globally in {}",
        AGENT_DIR
    );
    Ok(())
}

fn handle_sync_skills(root: &Path) -> Result<()> {
    if Path::new(AGENT_DIR).exists() {
        fs::remove_dir_all(AGENT_DIR).context("Failed to clean older .agent directory")?;
    }

    let src_path = root.join(KIT_AGENT_PATH);
    copy_dir_all(&src_path, AGENT_DIR)
        .with_context(|| format!("Failed to sync kit from {:?}", src_path))?;

    println!("✅ Skills & Agents synced successfully!");
    Ok(())
}

fn handle_doctor(root: &Path) -> Result<()> {
    println!("OrbitSmith Doctor");

    let tool = "git";
    let status = if is_tool_installed(tool) {
        "OK"
    } else {
        "MISSING"
    };
    println!("- {}: {}", tool, status);

    let kit_status = if root.join(KIT_AGENT_PATH).exists() {
        "OK"
    } else {
        "MISSING"
    };
    println!("- kit/.agent: {}", kit_status);

    Ok(())
}

fn handle_status() -> Result<()> {
    let agent_path = Path::new(AGENT_DIR);

    if !agent_path.exists() {
        println!("❌ Antigravity Kit is not initialized in this directory.");
        println!("   Run `orbitsmith init` to scaffold the .agent/ directory.");
        return Ok(());
    }

    println!("✅ Antigravity Kit Status:");

    for sub in ["agents", "skills", "workflows", "scripts"] {
        let p = agent_path.join(sub);
        let status = if p.exists() { "OK" } else { "MISSING" };
        println!(" - {}/: {}", sub, status);
    }

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
        Commands::Status => handle_status()?,
        Commands::Update => handle_update(&root)?,
    }

    Ok(())
}
