use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

type MinHeap<T> = BinaryHeap<Reverse<T>>;

fn read_lines<P>(input_file: P) -> anyhow::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(input_file)?;
    Ok(BufReader::new(file).lines())
}

fn read_lists<P>(input_file: P) -> (Vec<u32>, Vec<u32>)
where
    P: AsRef<Path>,
{
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let lines = read_lines(input_file).unwrap();
    for line in lines.flatten() {
        let columns: Vec<&str> = line.split_whitespace().collect();
        left_list.push(columns[0].parse().unwrap());
        right_list.push(columns[1].parse().unwrap());
    }

    (left_list, right_list)
}

fn build_min_heaps(
    left_list: &mut Vec<u32>,
    right_list: &mut Vec<u32>,
) -> (MinHeap<u32>, MinHeap<u32>) {
    let mut left_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    for val in left_list {
        left_heap.push(Reverse(*val));
    }
    for val in right_list {
        right_heap.push(Reverse(*val));
    }

    (left_heap, right_heap)
}

fn calc_tot_dist(heap_left: &mut MinHeap<u32>, heap_right: &mut MinHeap<u32>) -> u32 {
    let mut dist_tot: u32 = 0;
    while let Some(Reverse(val_left)) = heap_left.pop() {
        let Reverse(val_right) = heap_right.pop().unwrap();
        dist_tot += val_left.abs_diff(val_right);
    }

    dist_tot
}

fn calc_similarity_score(right_list: Vec<u32>, left_list: Vec<u32>) -> u32 {
    let mut number_counts: HashMap<u32, u32> = HashMap::new();
    for val in right_list {
        number_counts
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarty_score: u32 = 0;
    for val in left_list {
        let count = number_counts.get(&val).unwrap_or(&0);
        similarty_score += val * count;
    }

    similarty_score
}

fn main() {
    let (mut left_list, mut right_list) = read_lists("input.txt");
    let (mut left_heap, mut right_heap) = build_min_heaps(&mut left_list, &mut right_list);
    let dist_tot = calc_tot_dist(&mut left_heap, &mut right_heap);
    println!("Total distance is: {}", dist_tot);

    let similarity_score = calc_similarity_score(right_list, left_list);

    println!("Total similarty score is: {}", similarity_score);
}
