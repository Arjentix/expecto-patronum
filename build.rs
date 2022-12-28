use std::{
    env, fs,
    io::{self, BufRead as _},
    path::Path,
};

const ASSETS_PATH: &str = "resources/assets.txt";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={ASSETS_PATH}");

    let out_dir = env::var_os("OUT_DIR").expect("Failed to get `OUT_DIR` env var");
    let dest_path = Path::new(&out_dir).join("assets.rs");

    let asset_file = fs::File::open(ASSETS_PATH)
        .unwrap_or_else(|err| panic!("Failed to open asset file `{ASSETS_PATH}`: {err:?}"));
    let reader = io::BufReader::new(asset_file);

    let assets = collect_assets(reader);
    if assets.is_empty() {
        panic!("Assets file `{ASSETS_PATH}` contains no asset");
    }

    let code = generate_assets_code(assets);
    fs::write(dest_path, code).expect("Failed to write asset code");
}

fn collect_assets(reader: io::BufReader<fs::File>) -> Vec<String> {
    let mut asset = String::new();
    let mut assets: Vec<String> = reader
        .lines()
        .map(|l| {
            l.unwrap_or_else(|err| {
                panic!("Can't read line from asset file `{ASSETS_PATH}`: {err:?}")
            })
        })
        .filter_map(|l| {
            if l == "\n" || l.is_empty() {
                let res = Some(asset.clone());
                asset.clear();
                res
            } else {
                asset.push_str(&l);
                asset.push('\n');
                None
            }
        })
        .filter(|l| !l.is_empty())
        .collect();

    // Pushing last asset
    if !asset.is_empty() {
        assets.push(asset);
    }

    assets
}

fn generate_assets_code(assets: Vec<String>) -> String {
    let mut code = format!(
        "/// Array with patronus assets\n\
         const ASSETS: [&str; {}] = [\n",
        assets.len()
    );
    for asset in assets {
        code.push_str(&format!("r###\"{asset}\"###,\n"));
    }
    code.push_str("];");

    code
}
