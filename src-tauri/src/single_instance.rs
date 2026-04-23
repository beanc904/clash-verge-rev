use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt as _;

use crate::{
    feat,
    process::AsyncHandler,
    utils::notification::{NotificationEvent, notify_event},
};

pub fn setup_single_instance(app: &AppHandle, args: Vec<String>, _cwd: String) {
    match args.get(1) {
        Some(arg1) => match arg1.as_str() {
            "--dashboard-toggle" => {
                AsyncHandler::spawn(async move || {
                    feat::open_or_close_dashboard().await;
                    notify_event(NotificationEvent::DashboardToggled).await;
                });
            }
            "--toggle-proxy" => {
                AsyncHandler::spawn(async move || {
                    feat::toggle_system_proxy().await;
                    notify_event(NotificationEvent::SystemProxyToggled).await;
                });
            }
            "--toggle-tun-mode" => {
                AsyncHandler::spawn(async move || {
                    feat::toggle_tun_mode(None).await;
                    notify_event(NotificationEvent::TunModeToggled).await;
                });
            }
            "--clashmode-rule" => {
                AsyncHandler::spawn(async move || {
                    feat::change_clash_mode("rule".into()).await;
                    notify_event(NotificationEvent::ClashModeChanged { mode: "Rule" }).await;
                });
            }
            "--clashmode-global" => {
                AsyncHandler::spawn(async move || {
                    feat::change_clash_mode("global".into()).await;
                    notify_event(NotificationEvent::ClashModeChanged { mode: "Global" }).await;
                });
            }
            "--clashmode-direct" => {
                AsyncHandler::spawn(async move || {
                    feat::change_clash_mode("direct".into()).await;
                    notify_event(NotificationEvent::ClashModeChanged { mode: "Direct" }).await;
                });
            }
            _ => {
                app.notification()
                    .builder()
                    .title("Second Instance ERROR")
                    .body("ERROR: Run with unknown arg.")
                    .show()
                    .ok();
            }
        },
        None => {
            app.notification()
                .builder()
                .title("Second Instance ERROR")
                .body("ERROR: Run second instance without arg.")
                .show()
                .ok();
        }
    }
}
