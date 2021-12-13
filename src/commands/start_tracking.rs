pub async fn start_tracking(config: &fpm::Config, who: &str, whom: &str) -> fpm::Result<()> {
    tokio::fs::create_dir_all(format!("{}/.tracks", config.root.as_str()).as_str()).await?;

    let snapshots = fpm::snapshot::get_latest_snapshots(config).await?;
    check(config.root.as_str(), &snapshots, who, whom).await?;
    Ok(())
}

async fn check(
    base_path: &str,
    snapshots: &std::collections::BTreeMap<String, u128>,
    who: &str,
    whom: &str,
) -> fpm::Result<()> {
    let timestamp = match snapshots.contains_key(who) {
        Some(v) => v,
        None => {
            eprintln!("Error: {} is not synced yet", who);
            println!("Suggestion: Run `fpm sync` to sync the files");
            return Ok(());
        }
    };

    if !snapshots.contains_key(whom) {
        eprintln!("Error: {} is not synced yet", whom);
        println!("Suggestion: Run `fpm sync` to sync the files");
        return Ok(());
    }

    if who.contains('/') {
        let (dir, _) = who.rsplit_once('/').unwrap();
        std::fs::create_dir_all(format!("{}/.tracks/{}", base_path, dir))?;
    }

    let new_file_path = format!("{}/.tracks/{}", base_path, who.replace(".ftd", ".track"));

    write(whom, *timestamp, &new_file_path).await?;
    println!("{} is now tracking {}", who, whom);

    Ok(())
}

async fn write(whom: &str, timestamp: u128, path: &str) -> fpm::Result<()> {
    use tokio::io::AsyncWriteExt;
    let string = if exitst(whome) {
        let existing_doc = tokio::fs::read_to_string(&path).await?;
        format!(
            "{}\n\n-- fpm.track: {}\nself-timestamp: {}",
            existing_doc, whom, timestamp
        )
    } else {
        format!(
            "-- import: fpm\n\n-- fpm.track: {}\nself-timestamp: {}",
            whom, timestamp
        )
    };

    let mut f = tokio::fs::File::create(path).await?;
    f.write_all(string.as_bytes()).await?;
    Ok(())
}
