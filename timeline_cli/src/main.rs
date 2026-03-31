use std::{collections::HashSet, io::Write, path::PathBuf};

use anyhow::{Result, anyhow};
use clap::Parser;
use temporal_rs::{PlainDate, PlainDateTime, UtcOffset, ZonedDateTime};
use timeline_core::{TimelineManager, event::EventData, when::When};

fn main() -> Result<()> {
    println!("Welcome to the timeline creator CLI!");
    let mut manager = TimelineManager::new();
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line, &mut manager) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}

fn respond(line: &str, manager: &mut TimelineManager) -> Result<bool> {
    let args = shlex::split(line).ok_or(anyhow!("error: Invalid quoting"))?;

    let cli = Cli::try_parse_from(args)?;
    dbg!(&cli);
    match cli.command {
        Commands::Show => {
            for event in manager.ordered_events() {
                writeln!(std::io::stdout(), "{} - ({})", event.name(), event.when())?;

                if event.tags().is_empty() {
                    continue;
                }
                writeln!(std::io::stdout(), "\tTags:")?;
                for tag in event.tags() {
                    writeln!(std::io::stdout(), "\t\t{}", manager.tag_data(*tag).expect("Just got TagId from an event contained by the manager, so it should exist.").name())?;
                }
            }
            std::io::stdout().flush()?;
        }
        Commands::AddEvent { when, name } => {
            manager.add_event(EventData::new(
                When::instant(
                    ZonedDateTime::from_utf8(
                        when.as_bytes(),
                        temporal_rs::options::Disambiguation::Compatible,
                        temporal_rs::options::OffsetDisambiguation::Ignore,
                    )
                    .or_else(|_| {
                        PlainDateTime::from_utf8(when.as_bytes()).map_or_else(
                            |_| {
                                PlainDate::from_utf8(when.as_bytes())?.to_zoned_date_time(
                                    temporal_rs::TimeZone::UtcOffset(UtcOffset::from_minutes(0)),
                                    None,
                                )
                            },
                            |dt| {
                                dt.to_zoned_date_time(
                                    temporal_rs::TimeZone::UtcOffset(UtcOffset::from_minutes(0)),
                                    temporal_rs::options::Disambiguation::Compatible,
                                )
                            },
                        )
                    })?,
                ),
                name,
                HashSet::new(),
            ))?;
        }
        Commands::Save { out } => {
            std::fs::write(out, serde_json::to_string_pretty(manager)?)?;
        }
        _ => {}
    }
    Ok(false)
}

#[derive(Debug, clap::Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    AddEvent { when: String, name: String },
    Show,
    Save { out: PathBuf },
    AddTag,
    RemoveTag,
    RemoveEvent,
}

fn readline() -> Result<String> {
    write!(std::io::stdout(), "$ ")?;

    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
