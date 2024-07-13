// SPDX-License-Identifier: GPL-3.0-only

use app::{App, Page};
use cosmic::app::Settings;
/// The `app` module is used by convention to indicate the main component of our application.
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = vec![
        (Page::Page1, "🖖 Hello from libcosmic.".into()),
        (Page::Page2, "🌟 This is an example application.".into()),
        (
            Page::Page3,
            "🚧 The libcosmic API is not stable yet.".into(),
        ),
    ];

    let settings = Settings::default();

    cosmic::app::run::<App>(settings, input)?;

    Ok(())
}
