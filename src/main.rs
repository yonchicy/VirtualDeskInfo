use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::PathParseIconLocationA;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetForegroundWindow};
use windows::Win32::System::Threading::GetCurrentProcessId;

use windows::core::PCWSTR;

type GetTargetType = Arc<Mutex<Option<HWND>>>;
const TARGET_APP_TITLE: &str = "TaskManagerWindow";
use eframe::egui;
use egui::viewport::ViewportBuilder;
use winvd::{get_current_desktop, get_desktop_count, listen_desktop_events, DesktopEvent};
fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
fn windows_virtual_desktop() {
    // Desktop count
    println!("Desktops: {:?}", get_desktop_count().unwrap());

    // Go to second desktop, index = 1
    // switch_desktop(1).unwrap();

    // To listen for changes, use crossbeam, mpsc or winit proxy as a sender

    // Keep the _notifications_thread alive for as long as you wish to listen changes
    // std::thread::spawn(|| {
    //     for item in rx {
    //         println!("{:?}", item);
    //     }
    // });

    // Wait for keypress
    println!("⛔ Press enter to stop");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    // get windows virtual desktop id
    let (tx, rx) = std::sync::mpsc::channel::<DesktopEvent>();
    let _notifications_thread = listen_desktop_events(tx);
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_decorations(true),
        ..Default::default()
    };
    println!("pid:{}",unsafe {
        GetCurrentProcessId()
    });
    eframe::run_native(
        "my_app",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::new(rx)))),
    );
}

struct MyEguiApp {
    inited: bool,
    hwnd: Option<HWND>,
    receiver: Receiver<DesktopEvent>,
}

impl MyEguiApp {
    fn new(r: Receiver<DesktopEvent>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            hwnd: None,
            inited: false,
            receiver: r,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.inited {
            println!("update");
            if let Ok(new_text) = self.receiver.try_recv() {
                match new_text {
                    DesktopEvent::DesktopChanged { new, old } => {
                        println!("Desktop changed from {:?} to {:?}", old, new);
                        let idx = new.get_index().unwrap();
                        println!("move to new desktop: {}", idx);
                        winvd::move_window_to_desktop(new, &self.hwnd.unwrap());
                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.heading(idx.to_string());
                        });
                    }
                    event => {
                        println!("other event:{:?}", event);
                    }
                };
            }
            let idx = get_current_desktop().unwrap().get_index().unwrap();
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(idx.to_string());
            });
            ctx.request_repaint();
        } else {
            self.hwnd = get_HWND();
            self.inited = true;
        }
    }
}
fn str_to_pcwstr(s: &str) -> Vec<u16> {
    let result = s
        .to_string()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    result
}

pub fn get_HWND() -> Option<HWND> {
    // 示例：通过窗口标题查找当前应用程序的 HWND
    let mut hwnd: HWND = unsafe {
        FindWindowW(
            PCWSTR::null(), // 类名，可选
            PCWSTR::from_raw(str_to_pcwstr("my_app").as_ptr()),
        )
    };

    if hwnd.0 != 0 {
        println!("通过窗口标题找到当前窗口的 HWND 是: {:?}", hwnd);
        return Some(hwnd);
    } else {
        println!("无法通过窗口标题获取 HWND");
    }

    // 或者使用 GetForegroundWindow() 获取当前活动窗口的 HWND
    hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0 != 0 {
        println!("当前前台窗口的 HWND 是: {:?}", hwnd);
        return Some(hwnd);
    } else {
        println!("无法获取当前前台窗口的 HWND");
    }
    None
}
