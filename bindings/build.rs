fn main() {
    winrt::build!(
        windows::win32::{
            EnumWindows,
            K32EnumProcesses,
            OpenProcess,
            K32EnumProcessModules,
            K32GetModuleBaseNameW,
            GetWindowTextW
        }
    );
}
