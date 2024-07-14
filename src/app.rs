// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Application API example

use cosmic::app::{Command, Core};
use cosmic::{executor, iced, ApplicationExt, Element};
use rand::distributions::{Distribution, Uniform};

const WORDLIST: &str = include_str!("../assets/wordlist.txt");
const PASSWORD_LEN: usize = 8;

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    GeneratePass,
    ButtonPress(bool),
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    password: String,
    pressed: bool,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.AppDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits command on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = App {
            core,
            password: String::new(),
            pressed: false,
        };

        app.set_header_title("My Cosmic App".into());

        (app, Command::none())
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::GeneratePass => {
                self.password = Self::generate_password();
            }
            Message::ButtonPress(state) => {
                self.pressed = state;
            }
        }

        println!("{}", self.pressed);

        Command::none()
    }

    /// Creates a view after each update.
    /// This needs changed to put substance on each page
    fn view(&self) -> Element<Self::Message> {
        let txt = cosmic::widget::text(&self.password);

        let btn = cosmic::widget::button("Generate")
            .on_press_down(Message::ButtonPress(true))
            .on_press(Message::GeneratePass);

        let column = cosmic::widget::column().push(txt).push(btn);

        let centered = cosmic::widget::container(column.width(200))
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        Element::from(centered)
    }
}

impl App {
    fn generate_password() -> String {
        let mut rng = rand::thread_rng();

        let between = Uniform::from(0..7775);

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                WORDLIST
                    .lines()
                    .nth(between.sample(&mut rng))
                    .expect("index in range")
            })
            .collect::<Vec<_>>()
            .join("-");

        password
    }
}
