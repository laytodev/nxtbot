use faithe::global;
use winapi::shared::windef::HHOOK;

global! {
    pub extern HHK: HHOOK = "osclient.exe"%"48 8B 0D ? ? ? ? 48 85 C9 74 1A";
    pub extern LoginState: u32 = "osclient.exe"@ "C7 05 ? ? ? ? 09 00 00 00 E9 E0 05 00 00";
}

