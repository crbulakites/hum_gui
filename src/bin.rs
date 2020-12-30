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

extern crate iced;

use iced::{Button, Column, Element, Sandbox, Settings, Text};
use iced::text_input;
use iced::button;

pub fn main() -> iced::Result {
    HumGui::run(Settings::default())
}

#[derive(Default)]
struct HumGui {
    input: text_input::State,
    read_button: button::State,
    play_button: button::State,
    score_path: String,
    score: String,
}

#[derive(Debug, Clone)]
enum Message {
    LoadScore,
    PlayScore,
    ScorePathUpdated(String),
}

impl Sandbox for HumGui {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Hum Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadScore => {
                let score_contents = hum::hum_io::read(&self.score_path);
                self.score = score_contents;
            }
            Message::PlayScore => {
                hum::play(self.score.clone());
            }
            Message::ScorePathUpdated(score_path) => {
                self.score_path = score_path;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input = text_input::TextInput::new(
            &mut self.input,
            "Enter the path to the *.hum score file here...",
            &self.score_path,
            Message::ScorePathUpdated,
        );

        Column::new()
            .push(input)
            .push(
                Button::new(&mut self.read_button, Text::new("Load Score"))
                .on_press(Message::LoadScore)
            )
            .push(
                Button::new(&mut self.play_button, Text::new("Play Score"))
                .on_press(Message::PlayScore)
            )
            .push(Text::new(&self.score))
            .into()
    }
}

