use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // println!("{}", args.len());
    if args.len() != 2 {
        panic!("die");
    }

    let dlq_dir = &args[1];
    // println!("{}", dlq_dir);

    // https://stackoverflow.com/questions/26076005/how-can-i-list-files-of-a-directory-in-rust
    let paths = fs::read_dir(&dlq_dir).unwrap();

    for path_ in paths {
        let path_buf = path_.unwrap().path();
        // path_.unwrap().path() gives `PathBuf` struct
        // println!("{}", path_buf.display());

        // PathBuf to string
        // https://doc.rust-lang.org/std/path/struct.path_buf.html
        // https://stackoverflow.com/questions/37388107/how-to-convert-the-path_buf-to-string

        let path_str = path_buf.into_os_string().into_string().unwrap();
        // println!("{}", path_str);
        let filename = path_str.clone();

        // read using serde_json
        // https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
        let file = fs::File::open(path_str).expect("file should open read only");
        let j_val: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
        // println!("{}", json);

        // let first_name = json.get("FirstName").expect("file should have FirstName key");
        let bssh_run_id = j_val.get("id").expect("file should have id key");

        // let instrument_run_id = json.get("instrumentRunId").expect("file should have instrumentRunId key");

        // print optional wrap `Option<String>`
        // https://stackoverflow.com/questions/67411273/printing-optionstring-variable-gives-error-after-unwrapping-variable
        let instrument_run_id = j_val.get("instrumentRunId");

        let gds_folder_path = j_val.get("gdsFolderPath").expect("file should have gdsFolderPath key");

        // println!("{} {}", &bssh_run_id, Some(instrument_run_id));
        println!("{} {:?} {} {}", &bssh_run_id, instrument_run_id, &gds_folder_path, &filename);
    }
}
