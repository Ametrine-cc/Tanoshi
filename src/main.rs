/*
 * Copyright (C) 2026 Ametrine Foundation
 *
 * This file is part of Tanoshi.
 *
 * Tanoshi is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tanoshi is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with YourProject. If not, see <https://www.gnu.org/licenses/>.

 * SPDX-License-Identifier: GPL-3.0-or-later
*/

use chrono::Local as time;
use iced::widget::{container, row, text};
use iced::{Background, Color, Element, Length, Task, window};

mod window_rules;

fn main() -> iced::Result {
    // Call the function and unpack the Option
    match window_rules::get_combined_screen_size() {
        Some((x, y, width, height)) => {
            println!("Virtual Desktop Bounds Found:");
            println!("  Top-Left Origin: X = {}, Y = {}", x, y);
            println!("  Combined Size:   {}x{} pixels", width, height);
        }
        None => {
            println!("Could not retrieve monitor information.");
        }
    }

    iced::application(Tanoshi::new, Tanoshi::update, Tanoshi::view)
        .title("Tanoshi")
        .window_size((w as f32, 40.0))
        .decorations(false)
        .resizable(false)
        .level(window::Level::AlwaysOnTop)
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

struct Tanoshi {
    current_time: String,
}

impl Tanoshi {
    fn new() -> (Self, Task<Message>) {
        let current_time = time::now().format("%H:%M").to_string();
        (Self { current_time }, Task::none())
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let content = row![
            text("TITLE:[Tanoshi]").size(16),
            iced::widget::Space::new(),
            text(format!("TIME:[{}]", self.current_time)).size(16),
        ]
        .align_y(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}
