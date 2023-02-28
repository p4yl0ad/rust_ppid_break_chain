# RustWERBreakChain
"simple PoC for self-breaking own parent-child process chain" by gtworek.

Re-wrote it using the windows-sys Rust crate.


![source image](monkey.png)

## How this works

A registration for restart is placed with Windows Error Reporting which lets WERFault restart the application 
automatically in the event that the application crashes or hangs. 

A 62 second sleep is carried out to satisfy the following check detailed by the MSDN:
`"To prevent cyclical restarts, the system will only restart the application if it has been running for a minimum of 
60 seconds."`

Once the 62 seconds are up, a crash is forced (Can be carried out by writing to a NULL pointer, however you feel really)
An abort call is used to abort the app which results in WerFault.exe restarting the app with a new conhost.exe 
with the commandline "monkey".

The parent process WerFault.exe (i.e. PID 7680) shortly after gets closed resulting in our process having a 
Non-existent process (7680) as its parent living inside a new conhost.exe.


## Getting Started

### Dependencies
```
[dependencies.windows-sys]
version = "0.45.0"
features = [
    "Win32_Foundation",
    "Win32_System_Recovery",
]
```

### Executing program

building
```
cargo build --release
```
Running
```
cargo run --release
```

## Help

Make sure WER (Windows error reporting is active on the machine)
```
PS C:\Windows\system32> Enable-WindowsErrorReporting
True
```

## author
- PUNICODE_STRING

## Acknowledgments
Original discovery
* https://github.com/gtworek @0gtweet
* https://twitter.com/0gtweet/status/1629967880392458242