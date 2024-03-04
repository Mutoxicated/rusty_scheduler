use std::thread::sleep;
use std::time::Duration;

use windows::Win32::System::Console::SetConsoleTitleA;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, ShowWindow, SW_HIDE, SW_MINIMIZE, SW_SHOWNORMAL};
use windows::{core::PCSTR, Win32::Foundation::HWND};

unsafe impl Send for ConsoleWindow{}
unsafe impl Sync for ConsoleWindow{}

#[derive(Clone)]
pub struct ConsoleWindow {
    pub title: PCSTR,
    pub hwnd: HWND,
}

impl ConsoleWindow {
    pub fn init(title:PCSTR) -> Self {
        unsafe {
            SetConsoleTitleA(title).unwrap();

            sleep(Duration::new(0,600000000));

            let hwnd = FindWindowA(PCSTR::null(), title);

            Self {
                title,
                hwnd,
            }
        }
    }

    pub fn hide(&self){
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    pub fn show(&self){
        unsafe {
            ShowWindow(self.hwnd, SW_SHOWNORMAL);
        }
    }

    pub fn min(&self){
        unsafe {
            ShowWindow(self.hwnd, SW_MINIMIZE);
        }
    }
}