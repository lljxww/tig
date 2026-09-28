use anyhow::{Context, bail};
use chrono::{DateTime, FixedOffset};

pub fn get_log_format(timestamp: &str, timezone: &str) -> anyhow::Result<String> {
    let timestamp: i64 = timestamp.parse()?;

    if timezone.len() != 5 {
        bail!("错误的时区: {}", timezone);
    }

    let sign = match &timezone[..1] {
        "+" => 1,
        "-" => -1,
        _ => bail!("错误的时区: {}", timezone),
    };

    let hours: i32 = timezone[1..3].parse()?;
    let minutes: i32 = timezone[3..5].parse()?;

    if hours > 23 || minutes > 59 {
        bail!("错误的时区: {}", timezone);
    }

    let offset_secs = sign * (hours * 3600 + minutes * 60);
    let offset = FixedOffset::east_opt(offset_secs).context("错误的时区")?;

    let dt = DateTime::from_timestamp(timestamp, 0)
        .context("commit文件不正确")?
        .with_timezone(&offset);

    Ok(format!("{}", dt.format("%a %b %e %H:%M:%S %Y %z")))
}
