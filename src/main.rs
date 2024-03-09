use std::process::{Command, Output};
mod args;

#[derive(Debug, Clone)]
struct Commit {
    repo: String,
    hash: String,
    datetime: String,
    name: String,
}

struct IterationOutput {
    commit: Commit,
    stdout: String,
    stderr: String,
}

impl IterationOutput {
    pub fn new(commit: Commit, out: Output) -> Self {
        Self {
            commit,
            stdout: String::from_utf8(out.stdout).unwrap(),
            stderr: String::from_utf8(out.stderr).unwrap(),
        }
    }

    fn print_text(&self) {
        println!(
            "Commit [{}] ({}): {}",
            self.commit.hash, self.commit.datetime, self.commit.name
        );
        println!("{}", self.stdout);
        if !self.stderr.is_empty() {
            println!("{}", self.stderr);
        }
    }

    fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "repo": std::fs::canonicalize(self.commit.repo.clone()).unwrap().to_str().unwrap().to_string(),
            "hash": self.commit.hash,
            "datetime": self.commit.datetime,
            "name": self.commit.datetime,
            "stdout": self.stdout,
            "stderr": self.stderr
        })
    }

    fn as_csv(&self) -> Vec<String> {
        vec![
            self.commit.hash.clone(),
            self.commit.datetime.clone(),
            self.commit.name.clone(),
            std::fs::canonicalize(self.commit.repo.clone())
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            self.stdout.replace('\n', "\\n"),
            self.stderr.replace('\n', "\\n"),
        ]
    }
}

impl Commit {
    pub fn run_command(&self, command: &str) -> IterationOutput {
        checkout(&self.repo, &self.hash).unwrap();
        // todo : expose env vars
        let out = Command::new("sh")
            .current_dir(&self.repo)
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();

        IterationOutput::new(self.clone(), out)
    }
}

fn checkout(repo: &str, commit: &str) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .current_dir(repo)
        .arg("checkout")
        .arg(commit)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute git command",
        ))
    }
}

fn get_commit_list(repo: &str) -> Result<Vec<Commit>, std::io::Error> {
    let output = Command::new("git")
        .current_dir(repo)
        .arg("log")
        .arg("--pretty=format:%h - %ad - %s")
        .arg("--date=iso")
        .output()?;

    if output.status.success() {
        let mut commits = Vec::new();
        let out = String::from_utf8(output.stdout).unwrap();
        for line in out.lines() {
            let mut split = line.split(" - ");
            commits.push(Commit {
                repo: repo.to_string(),
                hash: split.next().unwrap().to_string(),
                datetime: split.next().unwrap().to_string(),
                name: split.next().unwrap().to_string(),
            });
        }
        Ok(commits)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute git command",
        ))
    }
}

fn is_repository_clean(repo: &str) -> bool {
    let output = Command::new("git")
        .current_dir(repo)
        .arg("status")
        .arg("--porcelain")
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap().is_empty()
}

enum OutMode {
    Text,
    Json,
    Csv,
}

impl OutMode {
    pub const fn new(json: bool, csv: bool) -> Self {
        if json {
            Self::Json
        } else if csv {
            Self::Csv
        } else {
            Self::Text
        }
    }
}

fn main() {
    let args = args::get_args();

    let repo = args.get_one::<String>("repository").unwrap();
    let allow_dirty = args.get_flag("allow-dirty");
    let command = if args.get_flag("script_file") {
        std::fs::read_to_string(args.get_one::<String>("command").unwrap()).unwrap()
    } else {
        args.get_one::<String>("command").unwrap().clone()
    };
    let outmode = OutMode::new(args.get_flag("json"), args.get_flag("csv"));

    let mut out = Vec::new();

    if is_repository_clean(repo) || allow_dirty {
        let commits = get_commit_list(repo).unwrap();
        for commit in commits {
            // todo : add colors
            out.push(commit.run_command(&command));
        }
        checkout(repo, "main").unwrap();

        match outmode {
            OutMode::Text => {
                for i in out {
                    i.print_text();
                }
            }
            OutMode::Json => {
                let json: Vec<_> = out.into_iter().map(|x| x.as_json()).collect();
                println!(
                    "{}",
                    serde_json::to_string(&serde_json::to_value(json).unwrap()).unwrap()
                );
            }
            OutMode::Csv => {
                let csv: Vec<Vec<String>> = out.into_iter().map(|x| x.as_csv()).collect();
                let mut wtr = csv::Writer::from_writer(std::io::stdout());

                wtr.write_record(["hash", "datetime", "name", "repo", "stdout", "stderr"])
                    .unwrap();

                for record in csv {
                    wtr.write_record(&record).unwrap();
                }

                wtr.flush().unwrap();
            }
        }
    } else {
        eprintln!("Repository is not clean. If you want to allow operating over an unclean repository, pass the `--allow-dirty` flag.");
    }
}
