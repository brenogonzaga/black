use anyhow::Result;
use black::BlackBox;

fn main() -> Result<()> {
    let logger = BlackBox::new("blackbox.log")?;

    logger.log_event("Application started")?;

    logger.log_error("Error: could not connect to the database")?;

    logger.log_event("Data processing complete")?;

    Ok(())
}
