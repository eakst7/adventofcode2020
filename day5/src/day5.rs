use std::fs;
use std::cmp;
use bitmaps::Bitmap;
use typenum::U1024;

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("IO error reading input data");

    return contents
}

fn seat_id(code : &str) -> u32 {
    let mut low_row = 0;
    let mut high_row = 127;
    let mut low_col = 0;
    let mut high_col = 7;
    for c in code.chars() {
        let rows_cnt = high_row - low_row + 1;
        let cols_cnt = high_col - low_col + 1;
        if c == 'F' {
            high_row = high_row - (rows_cnt / 2);
        } else if c == 'B' {
            low_row = low_row + (rows_cnt / 2);
        } else if c == 'L' {
            high_col = high_col - (cols_cnt / 2);
        } else if c == 'R' {
            low_col = low_col + (cols_cnt / 2);
        } else {
            panic!("Unexpected input");
        }

        //println!("{} rc {} cc {} hr {} lr {} hc {} lc {}", c, rows_cnt, cols_cnt, high_row, low_row, high_col, low_col);
    }
    return high_row * 8 + high_col;
}

fn find_solution_part_1() {
    let mut max_id = 0;
    for s in read_input().split('\n') {
        let seat_id = seat_id(s);
        //println!("Seat id for {} is {}", s, seat_id);

        max_id = cmp::max(max_id, seat_id);
    }

    println!("Max seat id is {}",max_id);
}

fn find_solution_part_2() {
    let mut max_id = 0;
    let mut min_id = u32::MAX;

    let mut bmap : Bitmap<U1024> = Bitmap::new();

    for s in read_input().split('\n') {
        let seat_id = seat_id(s);
       //println!("Seat id for {} is {}", s, seat_id);

        // Set the seat id as occupied
        bmap.set(seat_id as usize, true);

        max_id = cmp::max(max_id, seat_id);
        min_id = cmp::min(min_id, seat_id);
    }

    // For any seats below the min_id found,
    // treat as occupied
    for i in 0..(min_id+1) {
        bmap.set(i as usize, true);
    }

    // For any seats above the max_id found,
    // treat as occupied
    for i in max_id..1024 {
        bmap.set(i as usize, true);
    }

    // Invert the bitmap.  Now any 'on' bits
    // indicate an unoccupied seat.
    bmap.invert();

    // Find the first unoccupied seat id
    let index = bmap.first_index().unwrap();

    println!("Open seat id is {}", index);
}


fn main() {
    find_solution_part_1();
    find_solution_part_2();
}
