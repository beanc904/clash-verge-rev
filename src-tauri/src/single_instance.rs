use std::borrow::Cow;

use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt as _;

use crate::{
    core::handle,
    feat,
    process::AsyncHandler,
    utils::notification::{NotificationEvent, notify_event},
};

pub fn setup_single_instance(_app: &AppHandle, args: Vec<String>, _cwd: String) {
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
                    // notify_event(NotificationEvent::SystemProxyToggled).await;
                    notify(Cow::Borrowed("System Proxy"), Cow::Borrowed("Toggle system proxy"));
                });
            }
            "--toggle-tun-mode" => {
                AsyncHandler::spawn(async move || {
                    feat::toggle_tun_mode(None).await;
                    // notify_event(NotificationEvent::TunModeToggled).await;
                    notify(Cow::Borrowed("Tun Mode"), Cow::Borrowed("Toggle tun mode"));
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
                notify(
                    Cow::Borrowed("Second Instance ERROR"),
                    Cow::Borrowed("ERROR: Run with unknown arg."),
                );
            }
        },
        None => {
            notify(
                Cow::Borrowed("Second Instance ERROR"),
                Cow::Borrowed("ERROR: Run second instance without arg."),
            );
        }
    }
}

fn notify(title: Cow<'_, str>, body: Cow<'_, str>) {
    let app_handle = handle::Handle::app_handle();
    app_handle.notification().builder().title(title).body(body).show().ok();
}
