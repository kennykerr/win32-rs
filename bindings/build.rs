fn main() {
    winrt::build!(
        windows::win32::menu_rc::{
            EnumWindows,
            GetWindowTextW,
        }
        windows::win32::ps_api::{
            K32EnumProcesses,
            K32EnumProcessModules,
            K32GetModuleBaseNameW,
        }
        windows::win32::win_prog::{
            OpenProcess,
        }
        windows::win32::dwm::*
    );
}
