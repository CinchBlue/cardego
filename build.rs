use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("linux") | Ok("android") => {}
        Ok("freebsd") | Ok("dragonfly") => {}
        Ok("openbsd") | Ok("bitrig") | Ok("netbsd") | Ok("macos") | Ok("ios") => {}
        Ok("windows") => {
            // This is needed for Cairo. As of 19-07-2020, try and build GTK according
            // to https://github.com/wingtk/gvsbuild on Windows for this to run.
            println!(r"cargo:rustc-link-search=C:\gtk-build\gtk\x64\release\lib");
        }
        tos => panic!("unknown target os {:?}!", tos),
    }
}
