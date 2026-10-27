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

use winit::event_loop::EventLoop;

pub fn get_combined_screen_size() -> Option<(i32, i32, u32, u32)> {
    let event_loop = EventLoop::new().ok()?;

    let monitors: Vec<_> = event_loop.available_monitors().collect();
    if monitors.is_empty() {
        return None;
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for monitor in monitors {
        let pos = monitor.position();
        let size = monitor.size();

        let x = pos.x;
        let y = pos.y;
        let w = size.width as i32;
        let h = size.height as i32;

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + w);
        max_y = max_y.max(y + h);
    }

    let total_width = (max_x - min_x) as u32;
    let total_height = (max_y - min_y) as u32;

    Some((min_x, min_y, total_width, total_height))
}
