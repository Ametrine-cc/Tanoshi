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

use layer_shika::prelude::*;
use std::time::Duration;

fn main() -> Result<()> {
    // Load the shell
    let mut runtime = Shell::from_file("ui/shell.slint")
        .surface("DesktopShell")
        .height(36)
        .anchor(AnchorEdges::top_bar())
        .exclusive_zone(-1)
        .build()?;

    let event_loop = runtime.event_loop_handle();

    // Clock timer
    event_loop.add_timer(Duration::from_secs(1), |_deadline, app_state| {
        let time = chrono::Local::now().format("%H:%M").to_string();
        let value = layer_shika::slint_interpreter::Value::from(
            layer_shika::slint::SharedString::from(time),
        );

        for surface in app_state.all_outputs_mut() {
            let _ = surface
                .component_instance()
                .set_property("current-time", value.clone());
        }

        layer_shika::calloop::TimeoutAction::ToDuration(Duration::from_secs(1))
    })?;

    runtime.with_component("DesktopShell", |component| {
        let result = component.set_callback("launcher-clicked", |_args| {
            println!("Launcher button tapped");
            layer_shika::slint_interpreter::Value::Void
        });

        if let Err(error) = result {
            eprintln!("Could not register launcher callback: {error}");
        }
    });

    // Start Tenoshi shell
    runtime.run()?;
    Ok(())
}
