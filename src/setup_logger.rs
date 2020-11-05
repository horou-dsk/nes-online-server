use chrono::Local;

pub fn setup_logger() -> Result<(), fern::InitError> {
    // let colors = ColoredLevelConfig::new().debug(Color::Magenta);
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}:{}] [{}] {}",
                // colors.color(record.level()),
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.line().unwrap_or(0),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
