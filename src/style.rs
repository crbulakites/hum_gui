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

use iced::{button, Background, Color};


pub struct Button;


impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.25, 0.7))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
            text_color: Color::from_rgb(0.4, 0.4, 0.4),
            ..self.active()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb(0.25, 0.7, 1.0))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            ..self.active()
        }
    }
}
