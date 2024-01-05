use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{ChildStdin, ChildStdout, Stdio};
use std::time::Duration;

use ascii_table::AsciiTable;
use baz_core::{Board, Color, Winner};
use baz_dueler::deserialize_move;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    players: Vec<PlayerInfo>,
    tournament: Vec<String>,
}

impl Config {
    fn player(&self, player_name: &str) -> &PlayerInfo {
        self.players
            .iter()
            .find(|pi| pi.name == player_name)
            .unwrap_or_else(|| panic!("No player named {player_name}"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerInfo {
    name: String,
    args: Vec<String>,
    #[serde(flatten)]
    artifact_location: ArtifactLocation,
    build: String,
    artifact: PathBuf,
}

impl PlayerInfo {
    fn artifact_path(&self) -> PathBuf {
        PathBuf::from("players").join(&self.name)
    }
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

fn update_artifact_if_necessary(player: &PlayerInfo) {
    if !player.artifact_path().exists() {
        update_artifact(player);
    }
}

fn update_artifact(player: &PlayerInfo) {
    println!("Updating {}", player.name);
    match &player.artifact_location {
        ArtifactLocation::LocalArtifact { workdir } => build_and_copy(player, workdir),
        ArtifactLocation::Git { repo, target } => update_git_artifact(player, repo, target),
    }
}

fn update_git_artifact(player: &PlayerInfo, repo: &str, target: &str) {
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
    let original_dir = std::env::current_dir().expect("Failed to get the current dir");
    let player_file = original_dir.join(player.artifact_path());
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

#[derive(Default)]
struct MatchResult {
    white: usize,
    black: usize,
    draw: usize,
}
impl Debug for MatchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}({})", self.white, self.black, self.draw)
    }
}

fn play_match(
    games: usize,
    config: &Config,
    white_player_name: &str,
    black_player_name: &str,
) -> MatchResult {
    let mut result = MatchResult::default();
    for _ in 0..games {
        match play_game(config, white_player_name, black_player_name) {
            Winner::White => result.white += 1,
            Winner::Black => result.black += 1,
            Winner::Draw => result.draw += 1,
        }
    }
    result
}

fn play_game(config: &Config, white_player_name: &str, black_player_name: &str) -> Winner {
    println!("{white_player_name} vs. {black_player_name}");
    let white_player = config.player(white_player_name);
    let black_player = config.player(black_player_name);
    update_artifact_if_necessary(white_player);
    update_artifact_if_necessary(black_player);
    let mut white_process = std::process::Command::new(white_player.artifact_path())
        .args(&white_player.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start white player");
    let mut white_stdin = white_process.stdin.take().unwrap();
    let mut white_stdout = white_process.stdout.take().unwrap();
    let mut black_process = std::process::Command::new(black_player.artifact_path())
        .args(&black_player.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start white player");
    let mut black_stdin = black_process.stdin.take().unwrap();
    let mut black_stdout = black_process.stdout.take().unwrap();
    white_stdin
        .write_all("white\n".as_bytes())
        .expect("Failed to write to white process");
    black_stdin
        .write_all("black\n".as_bytes())
        .expect("Failed to write to black process");
    let mut board = Board::default();
    let mut current_color = Color::White;
    // TODO max turn cutoff results in draw
    while board.winner().is_none() {
        // println!("{board:?}");
        // std::thread::sleep(Duration::from_millis(1));
        board = match current_color {
            Color::White => play_turn(&board, &mut white_stdout, &mut black_stdin),
            Color::Black => play_turn(&board, &mut black_stdout, &mut white_stdin),
        };
        // std::thread::sleep(Duration::from_millis(1));
        current_color = current_color.invert();
    }
    white_process.kill().expect("Failed to kill white process");
    black_process.kill().expect("Failed to kill black process");
    board.winner().unwrap()
}

fn play_turn(board: &Board, stdout: &mut ChildStdout, stdin: &mut ChildStdin) -> Board {
    let mut buffer = String::new();
    let mut reader = BufReader::new(stdout);
    reader
        .read_line(&mut buffer)
        .expect("Failed to read move from player");
    let mov = deserialize_move(&buffer);
    // TODO verify move is legal
    let new_board = board.apply_move(&mov);
    if new_board.winner().is_none() {
        stdin
            .write_all(buffer.as_bytes())
            .expect("Failed to send move to player");
    }
    new_board
}

fn print_results(tournament: &[String], results: &HashMap<(&str, &str), MatchResult>) {
    let mut table = AsciiTable::default();
    for (i, player) in tournament.iter().enumerate() {
        table.column(i + 1).set_header(player);
    }
    let data: Vec<Vec<String>> = tournament
        .iter()
        .map(|white| {
            std::iter::once(white.to_string())
                .chain(tournament.iter().map(|black| {
                    if let Some(result) = results.get(&(white, black)) {
                        format!("{result:?}")
                    } else {
                        "".to_string()
                    }
                }))
                .collect()
        })
        .collect();
    table.print(data);
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
    Play {
        #[arg(long)]
        update: bool,
        #[arg(long)]
        skip_self: bool,
        games: usize,
    },
}

fn main() {
    // Create the git and players directories if they does not exist
    std::fs::create_dir_all("git").expect("Failed to create git dir");
    std::fs::create_dir_all("players").expect("Failed to create players dir");
    let args = Args::parse();
    let config = load_config(&args);
    match args.command {
        Commands::Update => {
            for player in config.players.iter() {
                update_artifact(player);
            }
        }
        Commands::Play {
            update,
            skip_self,
            games,
        } => {
            if update {
                for player_name in config.tournament.iter() {
                    update_artifact(config.player(player_name));
                }
            }
            let mut results: HashMap<(&str, &str), MatchResult> = HashMap::new();
            for white_player_name in config.tournament.iter() {
                for black_player_name in config.tournament.iter() {
                    if !(skip_self && white_player_name == black_player_name) {
                        results.insert(
                            (white_player_name, black_player_name),
                            play_match(games, &config, white_player_name, black_player_name),
                        );
                    }
                }
            }
            print_results(&config.tournament, &results);
        }
    }
}
