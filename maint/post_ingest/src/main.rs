use std::{env, fs};
use std::process::exit;

use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RunRecord {
    portal_run_id: String,
    ica_workflow_run_id: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("need input path");
        exit(0);
    }

    let mut loc = args[1].to_string();

    let pat = "**/*.json";  // https://github.com/rust-lang/glob

    loc.push_str(pat);

    for entry in glob(loc.as_str()).expect("Failed to read glob pattern") {
        // match entry {
        //     Ok(path) => println!("{:?}", path.display()),
        //     Err(e) => println!("{:?}", e),
        // }

        if let Ok(path_buf) = entry {
            // println!("{:?}", path_buf.display())
            // println!("{:?}", path_buf.file_name())
            // println!("{:?}", path_buf.parent())

            let file_name = path_buf.file_name().expect("file name error").to_str().unwrap();

            let path_parent = path_buf.parent().expect("parent path error").to_str().unwrap();
            let parents: Vec<&str> = path_parent.split("/").collect();
            // dbg!(parents);

            let subject_id = parents[parents.len() - 1];

            println!("{}: {}", subject_id, file_name);

            let file_content = fs::read_to_string(path_buf).expect("unable to read file");
            let rr: RunRecord = serde_json::from_str(&file_content).unwrap();
            // dbg!(rr);

            let wfr_id = rr.ica_workflow_run_id;
            println!("ica workflows runs get {}", wfr_id);

            // ah! need wes get - ok, back to py script for now

            break;
        }
    }
}
