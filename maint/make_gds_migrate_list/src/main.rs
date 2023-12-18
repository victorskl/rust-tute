use std::fs::read_to_string;
use std::path::Path;

fn main() {
    for line in read_to_string("../.nogit/fq_transfer_list.csv").unwrap().lines().skip(1) {
        // println!("{}", line.to_string());

        let row_vec: Vec<&str> = line.split(",").collect();
        // dbg!(row_vec);

        let id = row_vec[0];
        let read_1_path = Path::new(row_vec[5]);
        let read_2_path = Path::new(row_vec[6]);

        let src_path = read_1_path.parent().unwrap().display().to_string();
        let dst_path = read_1_path.parent().unwrap().display().to_string().replace("production", "staging");
        // println!("{}", dst_path);

        let r1 = read_1_path.file_name().unwrap().to_str().unwrap();  // :(
        let r2 = read_2_path.file_name().unwrap().to_str().unwrap();

        let out = format!("
gds-migrate \
    --src-project production \
    --src-path {src_path}/ \
    --dest-project staging \
    --dest-path {dst_path}/ \
    --rsync-args \"--include={r1},--include={r2},--exclude=*\" \
    2>&1 | tee gds-migrate.{id}.log");
        println!("{}", out);

        // break;
    }
}
