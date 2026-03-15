use std::error::Error;
use std::path::PathBuf;

// Adapted from: https://github.com/partydeck/partydeck/blob/main/src/util.rs
pub fn load_kwin_script_dbus(file: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Loading Splitscreen Kwin Script {}...", file.display());
    if !file.exists() {
        return Err("[partydeck] util::kwin_dbus_start_script - Script file doesn't exist!".into());
    }

    let conn = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )?;

    let _: i32 = proxy.call("loadScript", &(file.to_string_lossy(), "splitscreen"))?;
    println!("Splitscreen KwinScript loaded. Starting...");
    let _: () = proxy.call("start", &())?;

    println!("Splitscreen KWin script started.");
    Ok(())
}

pub fn unload_kwin_script_dbus() -> Result<(), Box<dyn Error>> {
    println!("Unloading splitscreen script...");
    let conn = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )?;

    let _: bool = proxy.call("unloadScript", &("splitscreen"))?;

    println!("Splitscreen KwinScript unloaded.");
    Ok(())
}
