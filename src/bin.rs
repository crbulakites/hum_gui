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

use iced::{Column, Element, Sandbox, Settings, Text};
use iced::text_input;

pub fn main() -> iced::Result {
    HumGui::run(Settings::default())
}

#[derive(Default)]
struct HumGui {
    input: text_input::State,
    score: String,
}

#[derive(Debug, Clone)]
enum Message {
    ScoreUpdated(String),
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
            Message::ScoreUpdated(score) => {
                self.score = score;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input = text_input::TextInput::new(
            &mut self.input,
            "Enter the score here...",
            &self.score,
            Message::ScoreUpdated,
        );

        Column::new()
            .push(input)
            .push(Text::new(&self.score))
            .into()
    }
}

