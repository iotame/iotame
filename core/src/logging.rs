use std::time::SystemTime;
use fern::{DateBased, Dispatch, InitError};
use fern::colors::{Color, ColoredLevelConfig};
use crate::config::IotameConfig;

pub(crate) fn setup_logger(config: &IotameConfig) -> Result<(), InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green);

    let log_path = config.environment_path.join("logs");
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
        .level(config.log_level)
        .chain(std::io::stdout())
        .chain(DateBased::new(log_path.join("iotame.log."), "%d%m%Y"))
        .apply()?;

    Ok(())
}
