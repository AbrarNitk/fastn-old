pub async fn update(config: &fpm::Config) -> fpm::Result<()> {
    if let Err(e) = std::fs::remove_dir_all(config.root.join(".packages")) {
        match e.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => return Err(e.into()),
        }
    };

    let c = fpm::Config::read().await?;
    if c.dependencies.is_empty() {
        println!("No dependencies to update.")
    } else if c.dependencies.len() == 1 {
        println!("Updated the package dependency.")
    } else {
        println!("Updated {} dependencies.", c.dependencies.len())
    }

    Ok(())
}