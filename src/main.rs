use std::fmt::Debug;
use std::io::Read;

use color_eyre::{Help, Report, SectionExt};
use tracing::debug;

fn decode_best_you_can(data: &[u8]) -> Result<Report, DebugString> {
    let x = Report::msg("could not find the right format");
    debug!("trying JSON");
    let x = x.section(
        Report::new(serde_json::from_slice::<serde_json::Value>(data).flip()?)
            .header("could not parse as JSON"),
    );
    debug!("trying YAML");
    let x = x.section(
        Report::new(serde_yaml::from_slice::<serde_yaml::Value>(data).flip()?)
            .header("could not parse as YAML"),
    );
    debug!("trying msgpack");
    let x = x.section(
        Report::new(rmp_serde::from_slice::<rmpv::Value>(data).flip()?)
            .header("could not parse as msgpack"),
    );
    Ok(x)
}

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = Vec::new();
    stdin.read_to_end(&mut input)?;
    let data = decode_best_you_can(&input).flip()?;
    println!("{}", data.0);
    Ok(())
}

trait Flip {
    type Flipped;

    fn flip(self) -> Self::Flipped;
}

impl<A, B> Flip for Result<A, B> {
    type Flipped = Result<B, A>;

    fn flip(self) -> Self::Flipped {
        match self {
            Ok(a) => Err(a),
            Err(b) => Ok(b),
        }
    }
}

struct DebugString(String);

impl<T: Debug> From<T> for DebugString {
    fn from(t: T) -> DebugString {
        DebugString(format!("{:#?}", t))
    }
}
