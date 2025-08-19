use std::path::PathBuf;
use fern::colors::{Color, ColoredLevelConfig};
use fern::{DateBased, Dispatch, InitError};
use std::time::SystemTime;

pub(crate) fn setup_logger(working_directory: PathBuf, log_level: log::LevelFilter) -> Result<(), InitError> {
    let colors = ColoredLevelConfig::new().info(Color::Green);

    let log_path = working_directory.join("logs");
    std::fs::create_dir_all(&log_path)?;

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(DateBased::new(log_path.join("iotame.log."), "%Y%m%d"))
        .apply()?;

    Ok(())
}
