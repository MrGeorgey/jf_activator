use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if std::env::consts::OS != "windows" {
        println!("This script is designed for and only works on Windows 10 and onwards.")
    } else {
        let mut winres = winresource::WindowsResource::new();
        winres.set_manifest(
            r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
            </assembly>"#,
        );
        winres.compile().unwrap();
    }
    println!("cargo:rerun-if-changed=jf_activator.toml");

    let out_dir = env::var("PROFILE").unwrap();
    let target_dir = Path::new("target").join(out_dir);

    if target_dir.exists() {
        let dest_path = target_dir.join("jf_activator.toml");

        fs::copy("jf_activator.toml", dest_path).unwrap();
    }
}
