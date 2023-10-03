// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::generated_code::{App, DateTimeAdapter};
use chrono::prelude::*;
use slint::*;

pub fn setup(window: &App) -> Timer {
    let update_timer = Timer::default();
    update_timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(300),
        {
            let weak_window = window.as_weak();

            move || {
                update(&weak_window.unwrap().global::<DateTimeAdapter>());
            }
        },
    );

    update_timer
}

fn update(header_adapter: &DateTimeAdapter) {
    let now = Local::now();

    header_adapter.set_date(slint::format!("{}", now.format("%a %e %b %Y")));
    header_adapter.set_time(slint::format!("{}", now.format("%H:%M:%S")));
}
