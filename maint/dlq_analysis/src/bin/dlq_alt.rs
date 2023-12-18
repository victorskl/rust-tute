use std::fs;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]  // require derive feature flag https://serde.rs/feature-flags.html
#[serde(rename_all = "camelCase")]  // PascalCase, camelCase https://serde.rs/attr-rename.html
struct SequenceRun {
    id: String,
    // instrument_run_id: String,
    status: String,
    gds_volume_name: String,
    gds_folder_path: String,
    // flowcell_barcode: String,
}

fn main() {
    let paths = fs::read_dir("../.nogit/DLQ").unwrap();

    for path in paths {
        // println!("{}", path.unwrap().path().display());

        let path_str = path.unwrap().path().into_os_string().into_string().unwrap();

        let mut file = fs::File::open(path_str).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let sqr: SequenceRun = serde_json::from_str(&buff).unwrap();

        let json = serde_json::to_string_pretty(&sqr).unwrap();
        println!("{}", json);

        // break;
    }
}
