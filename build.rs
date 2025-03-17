use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    let assets_src = PathBuf::from("assets");
    let assets_dest = target_dir.join("assets");

    if assets_dest.exists() {
        fs::remove_dir_all(&assets_dest).unwrap();
    }

    fs::create_dir_all(&assets_dest).unwrap();
    copy_dir(&assets_src, &assets_dest).unwrap();

    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rustc-link-search=native=D:\\msys2\\mingw64\\lib");
    println!("cargo:rustc-link-lib=SDL2");

}

fn copy_dir(src: &PathBuf, dest: &PathBuf) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
