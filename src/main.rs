// https://github.com/gtworek/PSBits/blob/master/Misc/BreakChain.c
// https://cs.github.com/microsoft/windows-rs/blob/371245db0fd703842b3f6351a8ea71aa46d1bd0e/crates/libs/windows/src/Windows/Win32/System/Recovery/mod.rs?q=language%3Arust+RegisterApplicationRestart#L48
// https://cs.github.com/microsoft/windows-rs?q=
// https://stackoverflow.com/questions/74518825/strange-process-names-when-trying-to-get-a-pid-in-rust-with-the-windows-api

/*
Full credits go to gtworek
+rep for the cool parent process de-chaining
*/
mod getppid;
use getppid::print_ppid;

use std::{
    env::args,
    process::{exit, abort},
    thread::sleep,
    time::Duration,
    process::{Command, ExitStatus},
    io::Result
};

use windows_sys::{
    Win32::System::Recovery::RegisterApplicationRestart,
    core::*,
    Win32::Foundation::S_OK,
};

static SLEEP_MS:Duration  = Duration::from_millis(62 * 1000);
static POST_SLEEP_MS:Duration  = Duration::from_millis(100000);

pub fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}

fn main() {
    let args:Vec<String> = args().collect();
    if args.len() == 1 {
        unsafe {
            let pcwstr_name:PCWSTR = w!("monkey");
            let h_result:HRESULT = RegisterApplicationRestart(
                pcwstr_name,
                0
            );
            if h_result != S_OK{
                println!("error {:?}", h_result);
                exit(h_result);
            }
            println!("Original parent, sleeping to make WER happy...");
            println!("Current process details and PPID...");
            print_ppid();
            sleep(SLEEP_MS);
            abort();
        }
    } else {
        println!("Monkey");
        println!("New process details and PPID...");
        print_ppid();
        sleep(POST_SLEEP_MS);
    }
}