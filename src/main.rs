
use log::info;
use winreg::RegKey;
use winreg::enums::*;

fn main() {

    env_logger::init();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key_7zip = hklm.open_subkey("SOFTWARE\\7-Zip").unwrap();
    let path: String = key_7zip.get_value("Path").unwrap();
    info!("{:?}", path);

    let dir_apple = hklm.open_subkey("SOFTWARE\\Apple Computer, Inc.").unwrap();
    for key in dir_apple.enum_keys() {
        info!("{:?}", key);
    }
}
