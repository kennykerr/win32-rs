use bindings::windows::win32;

fn main() {
    unsafe {
        enum_processes();
        win32::EnumWindows(Some(enum_window), 0);
    }
}

fn enum_processes() {
    unsafe {
        let mut processes: [u32; 1024] = [0; 1024];
        let mut bytes_needed = 0;

        win32::K32EnumProcesses(
            processes.as_mut_ptr(),
            processes.len() as u32,
            &mut bytes_needed,
        );

        for i in 0..bytes_needed / 4 {
            let process = win32::OpenProcess(
                win32::ProcessAccessRights::QueryInformation | win32::ProcessAccessRights::VmRead,
                0,
                processes[i as usize],
            );

            if process == 0 {
                continue;
            }

            let mut modules = 0;

            if win32::K32EnumProcessModules(
                process,
                &mut modules,
                std::mem::size_of::<isize>() as u32,
                &mut bytes_needed,
            ) != 0
            {
                let mut text: [u16; 512] = [0; 512];

                win32::K32GetModuleBaseNameW(
                    process,
                    modules,
                    text.as_mut_ptr(),
                    text.len() as u32 - 1,
                );

                let text = String::from_utf16_lossy(&text);
                println!("{}", text);
            }
        }
    }
}

extern "system" fn enum_window(window: isize, _: isize) -> i32 {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = win32::GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{}", text);
        }

        return 1;
    }
}
