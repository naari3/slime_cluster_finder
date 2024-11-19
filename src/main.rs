use std::{collections::HashSet, num::Wrapping, sync::Mutex, time::Instant};

use javarandom::JavaRandom;
use linya::{Bar, Progress};

mod javarandom;

fn generate_spiral(x_max: i32, y_max: i32) -> Vec<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (0, -1);
    let mut points = Vec::new();
    let limit: i64 = (x_max.max(y_max)).pow(2).into();

    for _ in 0..limit {
        if (-x_max / 2 < x && x <= x_max / 2) && (-y_max / 2 < y && y <= y_max / 2) {
            points.push((x, y));
        }

        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }

    points
}

fn generate_in_despawn_range_offsets(radius: i32) -> Vec<(i32, i32)> {
    let centers = [(0, 0), (0, -1), (-1, 0), (-1, -1)];
    centers
        .iter()
        .flat_map(|(cx, cy)| {
            (cx - radius..=cx + radius).flat_map(move |x| {
                (cy - radius..=cy + radius)
                    .filter(move |y| (x - cx) * (x - cx) + (y - cy) * (y - cy) < radius * radius)
                    .map(move |y| (x, y))
            })
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

const SLIME_CHUNK_NUMBER_A: Wrapping<i32> = Wrapping(0x4c1906);
const SLIME_CHUNK_NUMBER_B: Wrapping<i32> = Wrapping(0x5ac0db);
const SLIME_CHUNK_NUMBER_C: Wrapping<i64> = Wrapping(0x4307a7);
const SLIME_CHUNK_NUMBER_D: Wrapping<i32> = Wrapping(0x5f24f);
const SLIME_CHUNK_NUMBER_E: Wrapping<i64> = Wrapping(0x3ad8025f);

fn is_slime_chunk(seed: i64, x: i32, z: i32) -> bool {
    let seed = Wrapping(seed);
    let x = Wrapping(x);
    let z = Wrapping(z);

    let java_seed = (seed
        + Wrapping((x * x * SLIME_CHUNK_NUMBER_A).0 as i64)
        + Wrapping((x * SLIME_CHUNK_NUMBER_B).0 as i64)
        + Wrapping((z * z).0 as i64) * SLIME_CHUNK_NUMBER_C
        + Wrapping((z * SLIME_CHUNK_NUMBER_D).0 as i64))
        ^ SLIME_CHUNK_NUMBER_E;

    let mut rng = JavaRandom::new(java_seed.0);
    rng.next_int(10) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_slime_chunk() {
        assert!(is_slime_chunk(12345, 4, -3));
        assert!(is_slime_chunk(12345, 5, -3));
        assert!(is_slime_chunk(12345, 6, -2));
        assert!(!is_slime_chunk(12345, 6, -3));

        // further value, overflow test
        assert!(is_slime_chunk(12345, 3828, -3238));
        assert!(is_slime_chunk(12345, 15190, -14816));
        assert!(!is_slime_chunk(12345, 15190, -14817));

        assert!(is_slime_chunk(8011883210394390920, -2, 1));
        assert!(is_slime_chunk(8011883210394390920, -1, 0));
        assert!(!is_slime_chunk(8011883210394390920, 0, 0));
    }
}

fn main() {
    let seed = 8011883210394390920;
    let size = 10000;

    let start = Instant::now();
    let spiral = generate_spiral(size, size);
    let offsets = generate_in_despawn_range_offsets(7);
    let elapsed = start.elapsed();
    println!(
        "Time elapsed in generating spiral and offsets: {:?}",
        elapsed
    );

    let progress = Mutex::new(Progress::new());
    let bar: Bar = progress.lock().unwrap().bar(spiral.len(), "Seaching...");

    let start = Instant::now();
    let mut nums: Vec<((i32, i32), usize)> = spiral
        .iter()
        .map(|&chunk| {
            let slime_chunk_count = offsets
                .iter()
                .filter(|&&offset| {
                    let (x, z) = chunk;
                    let (ox, oz) = offset;
                    is_slime_chunk(seed, x + ox, z + oz)
                })
                .count();
            progress.lock().unwrap().inc_and_draw(&bar, 1);
            (chunk, slime_chunk_count)
        })
        .collect();
    let elapsed = start.elapsed();
    println!("Time elapsed in counting slime chunks: {:?}", elapsed);

    nums.sort_by_key(|&(_, count)| count);
    nums.reverse();

    for (chunk, count) in nums.iter().take(10) {
        println!("Chunk {:?} has {} slime chunks", chunk, count);
    }

    let max_chunk = nums.iter().max_by_key(|&(_, count)| count).unwrap();
    println!(
        "Max slime chunk is {:?} with count {}",
        max_chunk.0, max_chunk.1
    );

    offsets.iter().for_each(|&offset| {
        let (x, z) = max_chunk.0;
        let (ox, oz) = offset;
        let result = is_slime_chunk(seed, x + ox, z + oz);
        if result {
            println!("({}, {}): true", x + ox, z + oz);
        }
    });

    // println!("{:?}", spiral);
}
