use rustc_version::{version, version_meta, Channel};

fn main() {
    assert!(version().expect("Couldn't get compiler version!").major >= 1);
    if version_meta().expect("Couldn't get compiler metadata!").channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    }
}