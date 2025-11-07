use std::{array, sync::LazyLock};

type Peers = [[usize; 20]; 81];
type Units = [[usize; 27]; 81];

pub static PEERS: LazyLock<Peers> = LazyLock::new(|| array::from_fn(|idx| get_peers(idx)));
pub static UNITS: LazyLock<Units> = LazyLock::new(|| array::from_fn(|idx| get_units(idx)));

fn get_peers(idx: usize) -> [usize; 20] {
    let row = idx / 9;
    let col = idx % 9;

    let mut peers = [0; 20];

    let mut i = 0;

    for r in 0..9 {
        if r != row {
            peers[i] = r * 9 + col;
            i += 1;
        }
    }

    for c in 0..9 {
        if c != col {
            peers[i] = row * 9 + c;
            i += 1;
        }
    }

    let box_r = row / 3 * 3;
    let box_c = col / 3 * 3;

    for dr in 0..3 {
        for dc in 0..3 {
            let r = box_r + dr;
            let c = box_c + dc;

            if r != row && c != col {
                peers[i] = r * 9 + c;
                i += 1;
            }
        }
    }

    peers
}

fn get_units(idx: usize) -> [usize; 27] {
    let row = idx / 9;
    let col = idx % 9;

    let mut units = [0; 27];

    let mut i = 0;

    for r in 0..9 {
        units[i] = r * 9 + col;
        i += 1;
    }

    for c in 0..9 {
        units[i] = row * 9 + c;
        i += 1;
    }

    let box_r = row / 3 * 3;
    let box_c = col / 3 * 3;

    for dr in 0..3 {
        for dc in 0..3 {
            let r = box_r + dr;
            let c = box_c + dc;

            units[i] = r * 9 + c;
            i += 1;
        }
    }

    units
}
