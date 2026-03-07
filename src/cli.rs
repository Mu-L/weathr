use clap::Parser;
use clap_complete::Shell;

use crate::weather::WeatherCondition;

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n\nWeather data provided by Open-Meteo.com (https://open-meteo.com/)\n",
    "Data licensed under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)\n\n",
    "Geocoding powered by Nominatim/OpenStreetMap (https://nominatim.openstreetmap.org/)\n",
    "Data \u{00a9} OpenStreetMap contributors, ODbL (https://www.openstreetmap.org/copyright)"
);

const ABOUT: &str = concat!(
    "Terminal-based ASCII weather application\n\n",
    "Weather data provided by Open-Meteo.com (https://open-meteo.com/)\n",
    "Data licensed under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)\n\n",
    "Geocoding powered by Nominatim/OpenStreetMap (https://nominatim.openstreetmap.org/)\n",
    "Data \u{00a9} OpenStreetMap contributors, ODbL (https://www.openstreetmap.org/copyright)"
);

const CONDITION_GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "Clear Skies",
        &[
            ("clear", "Clear sunny sky"),
            ("partly-cloudy", "Partial cloud coverage"),
            ("cloudy", "Cloudy sky"),
            ("overcast", "Overcast sky"),
        ],
    ),
    (
        "Precipitation",
        &[
            ("fog", "Foggy conditions"),
            ("drizzle", "Light drizzle"),
            ("rain", "Rain"),
            ("freezing-rain", "Freezing rain"),
            ("rain-showers", "Rain showers"),
        ],
    ),
    (
        "Snow",
        &[
            ("snow", "Snow"),
            ("snow-grains", "Snow grains"),
            ("snow-showers", "Snow showers"),
        ],
    ),
    (
        "Storms",
        &[
            ("thunderstorm", "Thunderstorm"),
            ("thunderstorm-hail", "Thunderstorm with hail"),
        ],
    ),
];

#[derive(Parser)]
#[command(version, long_version = LONG_VERSION, about = ABOUT, long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "CONDITION",
        help = "Simulate weather condition (clear, rain, drizzle, snow, etc.)"
    )]
    pub simulate: Option<String>,

    #[arg(
        short,
        long,
        help = "Simulate night time (for testing moon, stars, fireflies)"
    )]
    pub night: bool,

    #[arg(short, long, help = "Enable falling autumn leaves")]
    pub leaves: bool,

    #[arg(long, help = "Auto-detect location via IP (uses ipinfo.io)")]
    pub auto_location: bool,

    #[arg(long, help = "Hide location coordinates in UI")]
    pub hide_location: bool,

    #[arg(long, help = "Hide HUD (status line)")]
    pub hide_hud: bool,

    #[arg(
        long,
        conflicts_with = "metric",
        help = "Use imperial units (°F, mph, inch)"
    )]
    pub imperial: bool,

    #[arg(
        long,
        conflicts_with = "imperial",
        help = "Use metric units (°C, km/h, mm)"
    )]
    pub metric: bool,

    #[arg(long, help = "Run silently (suppress non-error output)")]
    pub silent: bool,

    #[arg(long, value_name = "SHELL", value_enum)]
    pub completions: Option<Shell>,
}

pub enum SimulateError {
    UnknownCondition(String),
}

pub fn extract_simulate_missing_value(err: clap::Error) -> clap::Error {
    let msg = err.to_string();
    if msg.contains("--simulate") && msg.contains("value is required") {
        err
    } else {
        err.exit()
    }
}

pub fn validate_simulate(cli: &Cli) -> Result<(), SimulateError> {
    if let Some(condition) = &cli.simulate {
        condition
            .parse::<WeatherCondition>()
            .map_err(|_| SimulateError::UnknownCondition(condition.clone()))?;
    }
    Ok(())
}

pub fn print_simulate_help() {
    eprintln!("Available weather conditions:");
    for (group, conditions) in CONDITION_GROUPS {
        eprintln!();
        eprintln!("  {}:", group);
        for (name, description) in *conditions {
            eprintln!("    {:<18} - {}", name, description);
        }
    }
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  weathr --simulate rain");
    eprintln!("  weathr --simulate snow --night");
    eprintln!("  weathr -s thunderstorm -n");
}
