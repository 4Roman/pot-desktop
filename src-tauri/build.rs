fn main() {
    let mut attributes = tauri_build::Attributes::new();
    // On Windows, embed a custom application manifest that requests administrator
    // privileges (requireAdministrator). This makes pot's global hotkeys fire even
    // when an elevated window is in the foreground (Windows UIPI otherwise blocks
    // a non-elevated app's hotkeys over elevated windows). The manifest also keeps
    // the Common-Controls v6 dependency that Tauri's default manifest provides.
    #[cfg(windows)]
    {
        let windows =
            tauri_build::WindowsAttributes::new().app_manifest(include_str!("pot.manifest"));
        attributes = attributes.windows_attributes(windows);
    }
    tauri_build::try_build(attributes).expect("failed to run tauri build script");
}
