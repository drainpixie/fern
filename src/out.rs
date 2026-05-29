use colored::Colorize;

use crate::cli::Format;
use crate::layers::{Remote, RemoteStatus};

pub fn print_remotes(remotes: &[Remote], format: &Format) {
    let width = &remotes.iter().map(|r| r.name.len()).max().unwrap_or(0);

    match format {
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(remotes).expect("serialization failed")
            )
        }
        Format::Plain => remotes
            .iter()
            .for_each(|r| println!("{} {}", r.name, r.url)),
        Format::Text => {
            if remotes.is_empty() {
                println!("{}", "No remotes configured.".dimmed());
                return;
            }

            for r in remotes {
                println!(
                    "{} {}",
                    format!("{:<width$}", r.name).cyan().bold(),
                    r.url.dimmed(),
                );
            }
        }
    }
}

pub fn print_status(statuses: &[RemoteStatus], format: &Format) {
    match format {
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(statuses).expect("serialization failed")
            )
        }
        Format::Plain => {
            for s in statuses {
                if s.branches.is_empty() {
                    println!("{} (no tracked branches)", s.remote.name);
                }
                for b in &s.branches {
                    println!(
                        "{} {} ahead:{} behind:{}",
                        s.remote.name, b.local, b.ahead, b.behind
                    );
                }
            }
        }
        Format::Text => {
            if statuses.is_empty() {
                println!("{}", "No remotes configured.".dimmed());
                return;
            }

            for s in statuses {
                println!("{}  {}", s.remote.name.cyan().bold(), s.remote.url.dimmed());

                if s.branches.is_empty() {
                    println!("{}  {}", "└─".dimmed(), "no tracked branches".dimmed());
                } else {
                    let last = s.branches.len() - 1;

                    let local_width = s.branches.iter().map(|b| b.local.len()).max().unwrap_or(0);
                    let upstream_width = s
                        .branches
                        .iter()
                        .map(|b| b.upstream.len())
                        .max()
                        .unwrap_or(0);

                    for (i, b) in s.branches.iter().enumerate() {
                        let connector = if i == last { "└─" } else { "├─" };
                        let (dot, label) = sync_parts(b.ahead, b.behind);

                        println!(
                            "{}  {}  {}  {}  {dot} {label}",
                            connector.dimmed(),
                            format!("{:<local_width$}", b.local).bold(),
                            "→".dimmed(),
                            format!("{:<upstream_width$}", b.upstream).dimmed(),
                        );

                        if i == last {
                            println!();
                        }
                    }
                }
            }
        }
    }
}

fn sync_parts(ahead: usize, behind: usize) -> (String, String) {
    match (ahead, behind) {
        (0, 0) => ("●".green().to_string(), "up to date".dimmed().to_string()),
        (a, 0) => ("●".yellow().to_string(), format!("↑ {a} ahead")),
        (0, b) => ("●".red().to_string(), format!("↓ {b} behind")),
        (a, b) => ("●".red().to_string(), format!("↑ {a}  ↓ {b}")),
    }
}
