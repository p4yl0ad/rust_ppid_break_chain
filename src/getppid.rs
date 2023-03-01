// https://gist.github.com/mattn/253013/d47b90159cf8ffa4d92448614b748aa1d235ebe4
use std::process::exit;
use std::mem::size_of;
use windows_sys::Win32::{
    Foundation::FALSE,
    System::Threading::GetCurrentProcessId,
    System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot,
    System::Diagnostics::ToolHelp::Process32Next,
    System::Diagnostics::ToolHelp::TH32CS_SNAPPROCESS,
    System::Diagnostics::ToolHelp::PROCESSENTRY32,
    Foundation::INVALID_HANDLE_VALUE
};

const CT32S_FAILIURE: i32 = 0x00000001;
const P32N_FAILIURE: i32 = 0x00000003;

pub fn print_process_entry_32(process_entry_struct: PROCESSENTRY32) {
    println!("dwSize\t\t\t: {}", process_entry_struct.dwSize);
    println!("cntUsage\t\t: {}", process_entry_struct.cntUsage);
    println!("th32ProcessID\t\t: {}", process_entry_struct.th32ProcessID);
    println!("th32DefaultHeapID\t: {}", process_entry_struct.th32DefaultHeapID);
    println!("th32ModuleID\t\t: {}", process_entry_struct.th32ModuleID);
    println!("cntThreads\t\t: {}", process_entry_struct.cntThreads);
    println!("th32ParentProcessID\t: {}", process_entry_struct.th32ParentProcessID);
    println!("pcPriClassBase\t\t: {}", process_entry_struct.pcPriClassBase);
    println!("dwFlags\t\t\t: {}", process_entry_struct.dwFlags);
    println!("szExeFile\t\t: {}\n", std::str::from_utf8(&process_entry_struct.szExeFile).unwrap());
}

pub fn print_ppid() {
    let pid = unsafe {
        GetCurrentProcessId()
    };
    let hsnapshot = unsafe {
        CreateToolhelp32Snapshot(
            TH32CS_SNAPPROCESS,
            0
        )
    };
    if hsnapshot == INVALID_HANDLE_VALUE {
        println!("Cannot get snapshot : CreateToolhelp32Snapshot:hsnapshot == INVALID_HANDLE_VALUE");
        exit(CT32S_FAILIURE);
    }
    let mut pe32 = PROCESSENTRY32 {
        dwSize: 0u32,
        cntUsage: 0u32,
        th32ProcessID: 0u32,
        th32DefaultHeapID: 0usize,
        th32ModuleID: 0u32,
        cntThreads: 0u32,
        th32ParentProcessID: 0u32,
        pcPriClassBase: 0i32,
        dwFlags: 0u32,
        szExeFile: [0u8; 260]
    };
    let size_of_pe32_struct = size_of::<PROCESSENTRY32>();
    pe32.dwSize = size_of_pe32_struct as u32;
    let p32_return = unsafe {
        Process32Next(
            hsnapshot, // proc snapshot
            &mut pe32
        )
    };
    if p32_return == FALSE {
        eprintln!("Process32Next failed : Process32Next:p32_return == FALSE");
        exit(P32N_FAILIURE);
    }
    loop {
        if pe32.th32ProcessID == pid {
            let ppid = pe32.th32ParentProcessID;
            println!("PPID FOUND : {}, PID = {}",
                     pid, ppid
            );
            print_process_entry_32(pe32);
            break;
        }
        // null out everything but dwSize bit jankey but works
        //pe32.dwSize = 0u32;
        pe32.cntUsage = 0u32;
        pe32.th32ProcessID = 0u32;
        pe32.th32DefaultHeapID = 0usize;
        pe32.th32ModuleID = 0u32;
        pe32.cntThreads = 0u32;
        pe32.th32ParentProcessID = 0u32;
        pe32.pcPriClassBase = 0i32;
        pe32.dwFlags = 0u32;
        pe32.szExeFile = [0u8; 260];
        let p32_next_ret = unsafe {
            Process32Next(
                hsnapshot,
                &mut pe32
            )
        };
        if p32_next_ret == FALSE {
            eprintln!("Process32Next failed : Process32Next:p32_next_ret == FALSE");
            exit(P32N_FAILIURE);
        }
    }
}