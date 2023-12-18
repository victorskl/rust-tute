use std::path::Path;

use make_gds_migrate_list::read_lines;

fn main() {
    if let Ok(lines) = read_lines("../.nogit/fq_transfer_list.csv") {
        for line in lines.skip(1) {
            if let Ok(row) = line {
                // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
                // println!("{}", row);
                let row_vec: Vec<&str> = row.split(",").collect();
                // dbg!(row_vec);

                let id = row_vec[0];
                let read_1_path = Path::new(row_vec[5]);
                let read_2_path = Path::new(row_vec[6]);
                // println!("{:?}", read_2_path.file_name());

                let fwd_fq = read_1_path.file_name().unwrap().to_str().unwrap();
                let rev_fq = read_2_path.file_name().unwrap().to_str().unwrap();

                let src_path = read_1_path.parent().unwrap().display().to_string();
                let dst_path = read_1_path.parent().unwrap().display().to_string().replace("production", "staging");
                // println!("{}", dst_path);

                let out = format!("
gds-migrate \
    --src-project production \
    --src-path {src_path} \
    --dest-project staging \
    --dest-path {dst_path} \
    --rsync-args \"--include={fwd_fq},--include={rev_fq},--exclude=*\" \
    2>&1 | tee gds-migrate.{id}.log");
                println!("{}", out);

                // break;
            }
        }
    }
}
