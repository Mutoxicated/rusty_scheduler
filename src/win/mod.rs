use std::thread::sleep;
use std::time::Duration;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Console::SetConsoleTitleA;
use windows::Win32::UI::Shell::{ShellExecuteA, ShellExecuteW};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcA, FindWindowA, ShowWindow, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWDEFAULT, SW_SHOWNORMAL, WM_QUIT};
use windows::core::PCSTR;

use crate::pst_data::Data;

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
            ShowWindow(self.hwnd, SW_SHOWDEFAULT);
        }
    }

    pub fn min(&self){
        unsafe {
            ShowWindow(self.hwnd, SW_MINIMIZE);
        }
    }

    pub fn show_full(&self){
        unsafe {
            ShowWindow(self.hwnd, SW_MAXIMIZE);
        }
    }

    pub fn about(&self) {
        unsafe {
            ShellExecuteA(self.hwnd, PCSTR::from_raw("open\0".as_bytes().as_ptr()), PCSTR::from_raw("https://github.com/Mutoxicated/rusty_scheduler\0".as_bytes().as_ptr()), PCSTR::null(), PCSTR::null(), SW_SHOWNORMAL);
        }
    }
}