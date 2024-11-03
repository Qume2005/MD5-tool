mod gui;
mod core;

use gui::{subscription, update, view};

use anyhow::Result;
use iced::application;

pub fn run() -> Result<()> {
    application("MD5 tool", update, view)
        .subscription(subscription)
        .run()
        .map_err(|err| anyhow::Error::new(err))
}
