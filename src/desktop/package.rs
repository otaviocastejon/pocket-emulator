use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "macos")]
use pocketemulator::ui_icon;

pub fn package_current_platform() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;
    if !status.success() {
        return Err("release build failed".into());
    }

    let exe = std::env::current_exe()?;
    let bin_name = exe
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("pocketemulator")
        .to_string();
    let src = PathBuf::from("target").join("release").join(&bin_name);
    let dist = PathBuf::from("dist");
    fs::create_dir_all(&dist)?;
    let dst = dist.join(&bin_name);
    fs::copy(&src, &dst)?;
    println!("Packaged binary: {}", dst.display());

    #[cfg(target_os = "macos")]
    {
        let app_dir = dist.join("PocketEmulator.app");
        let contents = app_dir.join("Contents");
        let macos_dir = contents.join("MacOS");
        let resources_dir = contents.join("Resources");
        fs::create_dir_all(&macos_dir)?;
        fs::create_dir_all(&resources_dir)?;
        let app_bin = macos_dir.join("PocketEmulator");
        fs::copy(&src, &app_bin)?;
        let icon_dst = resources_dir.join("icon.png");
        fs::write(&icon_dst, ui_icon::icon_png_bytes())?;
        let icns_path = build_icns_from_embedded_png(&resources_dir)?;
        if !icns_path.exists() {
            return Err("failed to create PocketEmulator.icns".into());
        }

        let plist = contents.join("Info.plist");
        let plist_contents = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>PocketEmulator</string>
    <key>CFBundleExecutable</key>
    <string>PocketEmulator</string>
    <key>CFBundleIdentifier</key>
    <string>com.pocketemulator.app</string>
    <key>CFBundleIconFile</key>
    <string>PocketEmulator</string>
    <key>CFBundleIconName</key>
    <string>PocketEmulator</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>PocketEmulator</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
"#;
        fs::write(&plist, plist_contents)?;
        let sign = Command::new("codesign")
            .args([
                "--force",
                "--deep",
                "-s",
                "-",
                app_dir.to_string_lossy().as_ref(),
            ])
            .status();
        match sign {
            Ok(s) if s.success() => {}
            Ok(_) => eprintln!(
                "warning: codesign failed; if the app won't open from Finder, run:\n  codesign --force --deep -s - {}",
                app_dir.display()
            ),
            Err(e) => eprintln!("warning: could not run codesign: {e}"),
        }
        let _ = Command::new("touch").arg(&app_dir).status();
        println!("Packaged macOS app bundle: {}", app_dir.display());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn build_icns_from_embedded_png(
    resources_dir: &std::path::Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(ui_icon::icon_png_bytes())?.to_rgba8();
    let iconset_dir = resources_dir.join("PocketEmulator.iconset");
    if iconset_dir.exists() {
        let _ = fs::remove_dir_all(&iconset_dir);
    }
    fs::create_dir_all(&iconset_dir)?;

    let sizes: &[(u32, &str)] = &[
        (16, "icon_16x16.png"),
        (32, "icon_16x16@2x.png"),
        (32, "icon_32x32.png"),
        (64, "icon_32x32@2x.png"),
        (128, "icon_128x128.png"),
        (256, "icon_128x128@2x.png"),
        (256, "icon_256x256.png"),
        (512, "icon_256x256@2x.png"),
        (512, "icon_512x512.png"),
        (1024, "icon_512x512@2x.png"),
    ];

    for (size, name) in sizes {
        let resized = image::imageops::resize(
            &image,
            *size,
            *size,
            image::imageops::FilterType::CatmullRom,
        );
        resized.save(iconset_dir.join(name))?;
    }

    let icns_path = resources_dir.join("PocketEmulator.icns");
    let status = Command::new("iconutil")
        .arg("-c")
        .arg("icns")
        .arg(&iconset_dir)
        .arg("-o")
        .arg(&icns_path)
        .status()?;
    if !status.success() {
        return Err("iconutil failed to build .icns".into());
    }
    let _ = fs::remove_dir_all(iconset_dir);
    Ok(icns_path)
}
