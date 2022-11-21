use lenient_semver::parse;
use semver::Version;
use std::collections::HashMap;
use std::error::Error;
use std::process;
use std::io;

use serde::Deserialize;

/// This is a serde type for deserializing CSV into.
/// Unfortunately [semver::Version] doesn't have any serde attributes
/// so we need to do a two-step dance.
#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "OS version")]
    os_version: String,
    #[serde(rename = "Users")]
    users: i64,
    #[serde(rename = "New users")]
    new_users: i64,
    #[serde(rename = "Engaged sessions")]
    engaged_sessions: i64,
    #[serde(rename = "Event count")]
    event_count: i64,
}

/// This is basically the same type as above but with a parsed Version.
/// This can probably be better done with a newtype around [semver::Version]
/// and then implementing [serde::Deserialize] for that instead.
#[derive(Debug, Clone)]
struct VersionUsers {
    os_version: Version,
    num_users: i64,
    new_users: i64,
    engaged_sessions: i64,
    event_count: i64,
    fraction: f32,
}

impl VersionUsers {
    /// Merge two [VersionUsers] adding the values to self
    fn merge(&mut self, other: &Self) {
        self.num_users += other.num_users;
        self.new_users += other.new_users;
        self.engaged_sessions += other.engaged_sessions;
        self.event_count += other.event_count;
    }

    /// Get a new [VersionUsers] with the same values as before but
    /// with a new [semver::Version].
    fn with_version(&self, version: Version) -> Self {
        Self {
            os_version: version,
            num_users: self.num_users,
            new_users: self.new_users,
            engaged_sessions: self.engaged_sessions,
            event_count: self.event_count,
            fraction: 0f32,
        }
    }
}

impl From<Record> for VersionUsers {
    fn from(record: Record) -> Self {
        Self {
            os_version: parse(&record.os_version).unwrap(),
            num_users: record.users,
            new_users: record.new_users,
            engaged_sessions: record.engaged_sessions,
            event_count: record.event_count,
            fraction: 0f32,
        }
    }
}

fn main() {
    if let Err(err) = parse_csv() {
        println!("Error handling CSV: {}", err);
        process::exit(1);
    }
}

fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut versions_users: Vec<VersionUsers> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        versions_users.push(record.into());
    }
    let mut major_version_group: HashMap<Version, VersionUsers> = HashMap::new();
    for vu in &versions_users {
        add_by_major_version(&mut major_version_group, vu.clone());
    }
    {
        let mut collected: Vec<VersionUsers> = Vec::new();
        for vu in &major_version_group {
            collected.push(vu.1.clone());
        }
        collected.sort_by(|v1, v2| v2.os_version.cmp(&v1.os_version));
        let total: i64 = collected.iter().map(|v| v.num_users).reduce(|acc, num| acc + num).unwrap();
        for vu in &mut collected {
            vu.fraction = vu.num_users as f32 / total as f32;
        }
        println!(
            "{0: <9} {1: <7} {2: >4}   {3: <9} {4: <9}",
            "Version", "Users", "Pct.", "New users", "Sessions"
        );
        for vu in &collected {
            println!(
                "{0: <9} {1: <7} {2:>4.1}%  {3: <9} {4: <9}",
                vu.os_version, vu.num_users, vu.fraction * 100f32, vu.new_users, vu.engaged_sessions
            );
        }
    }
    Ok(())
}

/// Helper for adding multiple mixed major-minor-patch [VersionUsers] into a common [VersionUsers]
/// that only has a major version.
fn add_by_major_version(data: &mut HashMap<Version, VersionUsers>, version_users: VersionUsers) {
    let version = Version::new(version_users.os_version.major, 0, 0);
    data.entry(version.clone())
        .and_modify(|prev| prev.merge(&version_users))
        .or_insert(version_users.with_version(version));
}
