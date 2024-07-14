// SPDX-License-Identifier: GPL-3.0-only

use app::App;
use cosmic::app::Settings;
/// The `app` module is used by convention to indicate the main component of our application.
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default();

    cosmic::app::run::<App>(settings, ())?;

    Ok(())
}
