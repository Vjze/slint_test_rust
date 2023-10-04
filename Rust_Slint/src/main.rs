#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tool_slint::{datetime, get_result, *};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
#[tokio::main]
async fn main() {
    let app = App::new().unwrap();
    let cargo_worker = get_result::CargoWorker::new(&app);
    let _timer = datetime::setup(&app);

    app.global::<InfosData>().on_query({
        let window = app.as_weak();
        let cargo_channel = cargo_worker.channel.clone();
        move |action| {
            if action.text.len() == 0 {
                window.unwrap().invoke_show_confirm_popup();
            } else {
                cargo_channel
                    .send(get_result::QueryMessage::Action { action })
                    .unwrap()
            }
        }
    });
    app.global::<InfosData>().on_sort_descending({
        let cargo_channel = cargo_worker.channel.clone();
        move |index| {
            cargo_channel
                .send(get_result::QueryMessage::SortDescending { index })
                .unwrap()
        }
    });
    app.global::<InfosData>().on_sort_ascending({
        let cargo_channel = cargo_worker.channel.clone();
        move |index| {
            cargo_channel
                .send(get_result::QueryMessage::SortAscending { index })
                .unwrap()
        }
    });
    let window = app.as_weak();
    window
        .unwrap()
        .global::<InfosData>()
        .set_version(APP_VERSION.into());
    app.run().unwrap();
    cargo_worker.join().unwrap();
}
