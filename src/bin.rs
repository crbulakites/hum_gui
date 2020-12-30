/*
Hum GUI: A GUI for the Hum Notation Language and Synthesizer
Copyright (C) 2020 Connor R. Bulakites
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate hum;
extern crate iced;

use std::env;

use iced::{
    button,
    executor,
    scrollable,
    text_input,
    Application,
    Button,
    Command,
    Element,
    Scrollable,
    Settings,
    Text,
};

pub fn main() -> iced::Result {
    HumGui::run(Settings::default())
}

#[derive(Default)]
struct HumGui {
    scroll_state: scrollable::State,
    score_path_input: text_input::State,
    score_path: String,
    score: String,
    read_button: button::State,
    play_button: button::State,
    playback_state: bool,
}

#[derive(Debug, Clone)]
enum Message {
    UpdateScorePath(String),
    LoadScore,
    StartPlayback,
    StopPlayback,
}

impl Application for HumGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (HumGui, Command<Message>) {
        (
            HumGui {
                scroll_state: scrollable::State::new(),
                score_path_input: text_input::State::new(),
                score_path: String::new(),
                score: String::new(),
                read_button: button::State::new(),
                play_button: button::State::new(),
                playback_state: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hum Editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UpdateScorePath(score_path) => {
                self.score_path = score_path;
                Command::none()
            }
            Message::LoadScore => {
                let cwd = env::current_dir().unwrap();
                let complete_path = format!("{}/{}", cwd.display(), self.score_path);
                
                let score_contents = hum::hum_io::read(&complete_path);
                self.score = score_contents;
                Command::none()
            }
            Message::StartPlayback => {
                match self.playback_state {
                    false => {
                        self.playback_state = true;
                        Command::perform(
                            HumGui::play_in_background(self.score.clone()),
                            |_| Message::StopPlayback,
                        )
                    }
                    true => {
                        Command::none()
                    }
                }
            }
            Message::StopPlayback => {
                self.playback_state = false;
                Command::none()
            } 
        }
    }

    fn view(&mut self) -> Element<Message> {
        let score_path_input = text_input::TextInput::new(
            &mut self.score_path_input,
            "Enter the path to the *.hum score file here...",
            &self.score_path,
            Message::UpdateScorePath,
        );

        let playback_state = if self.playback_state { "Streaming" } else { "Idle" };

        Scrollable::new(&mut self.scroll_state)
            .push(score_path_input)
            .push(
                Button::new(&mut self.read_button, Text::new("Load Score"))
                .on_press(Message::LoadScore)
            )
            .push(
                Button::new(&mut self.play_button, Text::new("Play Score"))
                .on_press(Message::StartPlayback)
            )
            .push(Text::new(format!("Playback: {}", playback_state)))
            .push(Text::new(&self.score))
            .into()
    }
}


impl HumGui {
    async fn play_in_background(score: String) {
        println!("Playback started");
        match hum::play(score) {
            Ok(_) => println!("Playback completed successfully"),
            Err(_) => println!("Playback failed"),
        }
    }
}

