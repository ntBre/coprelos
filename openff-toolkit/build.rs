use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CONDA_PREFIX");
    let Ok(prefix) = env::var("CONDA_PREFIX") else {
        eprintln!(
            "must build inside a conda environment containing openff-toolkit"
        );
        std::process::exit(1);
    };
    println!("cargo:rustc-env=LD_LIBRARY_PATH={prefix}/lib");
}
