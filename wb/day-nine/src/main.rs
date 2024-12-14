use std::fs;

#[cfg(feature = "dev")]
const FILE_NAME: &str = "test.txt";
#[cfg(feature = "prod")]
const FILE_NAME: &str = "input.txt";

fn read_disk_map(path: &str) -> Vec<i32> {
    let file = fs::read_to_string(path).expect("Could not read file");
    let mut id: i32 = 0;
    let is_file_id = |index: usize, id: i32| if index % 2 == 0 { id } else { -1 };
    let mut disk_map = Vec::new();

    for (i, c) in file.chars().enumerate() {
        if i % 2 != 0 {
            id += 1;
        }
        let c_dig = c.to_digit(10).unwrap() as i32;
        disk_map.append(&mut vec![is_file_id(i, id); c_dig as usize]);
    }
    disk_map
}

fn visualize_disk_map(disk_map: &Vec<i32>) {
    for x in disk_map {
        if *x == -1 {
            print!("{}", '.')
        } else {
            print!("{}", x);
        }
    }
    println!();
}

fn split_files_and_spaces(disk_map: &Vec<i32>) -> (Vec<usize>, Vec<usize>) {
    let empty_space: Vec<(usize, &i32)> =
        disk_map.iter().enumerate().filter(|x| *x.1 == -1).collect();
    let files: Vec<(usize, &i32)> = disk_map.iter().enumerate().filter(|x| *x.1 != -1).collect();
    (
        files.iter().map(|&(index, _)| index).collect(),
        empty_space.iter().map(|&(index, _)| index).collect(),
    )
}

fn compact_disk(disk: &mut Vec<i32>, files: Vec<usize>, empty_spaces: Vec<usize>) -> Vec<i32> {
    let mut file_index: usize = files.len() - 1;

    for space_pos in empty_spaces {
        let file_pos = files[file_index] as usize;
        if file_pos <= space_pos {
            file_index += 1;
            break;
        }
        log::debug!("File index: {}, space index: {}", file_pos, space_pos);
        disk[space_pos] = disk[file_pos];
        file_index -= 1;
    }
    disk[0..files[file_index]].to_vec()
}

fn checksum(disk: &Vec<i32>) -> u64 {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (x_i, x_val)| acc + (x_i as u64) * *x_val as u64)
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut disk_map = read_disk_map(FILE_NAME);

    #[cfg(feature = "dev")]
    visualize_disk_map(&disk_map);

    let (files, empty_spaces) = split_files_and_spaces(&disk_map);

    let compacted_disk = compact_disk(&mut disk_map, files, empty_spaces);

    visualize_disk_map(&compacted_disk);

    log::info!("The checksum is: {}", checksum(&compacted_disk));
}
