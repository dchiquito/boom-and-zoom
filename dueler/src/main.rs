use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    players: Vec<PlayerInfo>,
    tournament: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerInfo {
    name: String,
    args: String,
    #[serde(flatten)]
    artifact_location: ArtifactLocation,
    build: String,
    artifact: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
enum ArtifactLocation {
    #[serde(rename = "git")]
    Git { repo: String, target: String },
    #[serde(untagged)]
    LocalArtifact { workdir: PathBuf },
}

fn load_config(_args: &Args) -> Config {
    let file = File::open("config.yml").expect("Failed to open config.yml");
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).expect("Failed to read config file")
}

fn update_artifact(player: &PlayerInfo) {
    match &player.artifact_location {
        ArtifactLocation::LocalArtifact { workdir } => build_and_copy(player, workdir),
        ArtifactLocation::Git { repo, target } => update_git_artifact(player, repo, target),
    }
}

fn update_git_artifact(player: &PlayerInfo, repo: &str, target: &str) {
    std::fs::create_dir_all("git").expect("Failed to create git dir");
    let repo_dir = PathBuf::from(format!("git/{}", player.name));
    if !repo_dir.exists()
        && !std::process::Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(&repo_dir)
            .status()
            .expect("Failed to clone git repo")
            .success()
    {
        panic!("Failed to clone git repo {repo}")
    }
    if !std::process::Command::new("git")
        .arg("pull")
        .status()
        .expect("Failed to git pull")
        .success()
    {
        panic!("Failed to git pull")
    }
    if !std::process::Command::new("git")
        .arg("checkout")
        .arg(target)
        .status()
        .expect("Failed to git checkout")
        .success()
    {
        panic!("Failed to git checkout {target}")
    }
    build_and_copy(player, &repo_dir);
}

fn build_and_copy(player: &PlayerInfo, build_dir: &Path) {
    // Create the players directory if it does not exist
    std::fs::create_dir_all("players").expect("Failed to create git dir");
    let original_dir = std::env::current_dir().expect("Failed to get the current dir");
    let player_file = original_dir.join("players").join(&player.name);
    // Navigate to the build directory
    std::env::set_current_dir(build_dir).expect("Failed to navigate to the build directory");
    // Run the build script
    if !std::process::Command::new("sh")
        .arg("-c")
        .arg(&player.build)
        .status()
        .expect("Failed to run build")
        .success()
    {
        panic!("Build failed for {}", player.name)
    }
    // Copy the build artifact
    std::fs::copy(&player.artifact, player_file).unwrap();
    // Go back to the original directory
    std::env::set_current_dir(original_dir)
        .expect("Failed to navigate back to the working directory");
}

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Update,
    Play,
}

fn main() {
    let args = Args::parse();
    let config = load_config(&args);
    match args.command {
        Commands::Update => {
            for player in config.players.iter() {
                update_artifact(player);
            }
        }
        Commands::Play => todo!(),
    }
    println!("{config:?}");
}
