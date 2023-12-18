use std::path::Path;

use make_gds_migrate_list::read_lines;

#[derive(Debug)]
struct FastqListRow {
    id: u64,
    rgid: String,
    rgsm: String,
    rglb: String,
    lane: u8,
    read_1: String,
    read_2: String,
    sequence_run_id: u64,
}

fn main() {
    let mut fqlr_vec = Vec::new();

    if let Ok(lines) = read_lines("../.nogit/fq_transfer_list.csv") {
        for line in lines.skip(1) {
            if let Ok(row) = line {
                // println!("{}", row);

                let row_vec: Vec<&str> = row.split(",").collect();
                // dbg!(row_vec[5]);

                let fqlr = FastqListRow {
                    id: row_vec[0].parse().unwrap(),
                    rgid: row_vec[1].to_string(),
                    rgsm: row_vec[2].to_string(),
                    rglb: row_vec[3].to_string(),
                    lane: row_vec[4].parse().unwrap(),
                    read_1: row_vec[5].to_string(),
                    read_2: row_vec[6].to_string(),
                    sequence_run_id: row_vec[7].parse().unwrap(),
                };
                // println!("{:?}", fqlr);

                fqlr_vec.push(fqlr);

                //break;
            }
        }
    }

    // dbg!(fqlr_vec);

    for fqlr in fqlr_vec {
        println!("{}, {}, {}, {}, {}, {}, {}, {}", fqlr.id, fqlr.rgid, fqlr.rgsm, fqlr.rglb, fqlr.lane, fqlr.read_1, fqlr.read_2, fqlr.sequence_run_id);

        let path = Path::new(&fqlr.read_1);
        for component in path.components() {
            println!("{component:?}");
        }

        break;
    }
}
