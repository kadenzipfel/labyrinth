use ethers::{
    abi::{encode, Token},
    types::{H160, U256},
};
use std::collections::VecDeque;
use std::env;
use tiny_keccak::{Hasher, Keccak};

// Constants for the size of the labyrinth and the maximum number of steps
const N: usize = 16;
const MAX_STEPS: usize = 64;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <address> <block_number>", args[0]);
        return;
    }

    let address = &args[1];
    let mut block_number: u64 = args[2].parse().expect("Invalid block number");

    // Main loop
    loop {
        // Generate the seed from the address and block number
        let seed = abi_encoded_keccak256(address, block_number);
        // Generate the labyrinth from the seed
        let labyrinth = generate_labyrinth(&seed);
        // Try to find a path through the labyrinth
        if let Some((path, directions)) = find_path(&labyrinth) {
            println!(
                "\nSolution found at block number {}: {:?}",
                block_number, path
            );
            println!("\nSeed: {:x?}", hex::encode(&seed));
            println!("\nGenerated labyrinth:\n");
            print_labyrinth(&labyrinth, &path);
            println!("\nDirections: {:x}", directions);
            break;
        }

        // Increment the block number for the next iteration
        block_number += 1;
    }
}

// Function to generate the seed from the address and block number
fn abi_encoded_keccak256(address: &str, block_number: u64) -> Vec<u8> {
    let address = address.parse::<H160>().expect("Invalid address");
    let block_number = U256::from(block_number);
    let encoded = encode(&[Token::Address(address), Token::Uint(block_number)]);
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(&encoded);
    hasher.finalize(&mut output);
    output.to_vec()
}

// Function to generate the labyrinth from the seed
fn generate_labyrinth(seed: &[u8]) -> [[bool; N]; N] {
    let mut labyrinth = [[false; N]; N];
    for i in 0..N * N {
        let byte = seed[i / 8];
        let bit = (byte >> (7 - i % 8)) & 0x1;
        let x = N - 1 - i / N;
        let y = N - 1 - i % N;
        labyrinth[x][y] = bit == 1;
    }
    labyrinth
}

// Function to find a path through the labyrinth
fn find_path(labyrinth: &[[bool; N]; N]) -> Option<(Vec<(usize, usize)>, u128)> {
    let mut visited = [[false; N]; N];
    let mut queue = VecDeque::new();
    let path = vec![(0, 0)];
    let directions: u128 = 0;
    let directions_index = 0;
    queue.push_back((0, 0, 0, path, directions, directions_index));
    while let Some((y, x, steps, path, directions, directions_index)) = queue.pop_front() {
        if y == N - 1 && x == N - 1 {
            return Some((path, directions));
        }
        // Check all four possible directions
        for &(dy, dx) in &[(0, -1), (1, 0), (-1, 0), (0, 1)] {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            // Check if the new position is within the labyrinth and is not a wall
            if ny >= 0 && ny < N as i32 && nx >= 0 && nx < N as i32 {
                let ny = ny as usize;
                let nx = nx as usize;
                if !labyrinth[ny][nx] && !visited[ny][nx] && steps + 1 <= MAX_STEPS {
                    visited[ny][nx] = true;
                    let mut new_path = path.clone();
                    new_path.push((ny, nx));
                    // Map the (dy, dx) pairs to the custom direction format
                    let direction = match (dy, dx) {
                        (-1, 0) => 0, // Up
                        (0, 1) => 1,  // Right
                        (0, -1) => 2, // Left
                        (1, 0) => 3,  // Down
                        _ => unreachable!(),
                    };
                    let new_directions = directions | ((direction as u128) << directions_index);
                    let new_directions_index = directions_index + 2;
                    queue.push_back((
                        ny,
                        nx,
                        steps + 1,
                        new_path.clone(),
                        new_directions,
                        new_directions_index,
                    ));
                }
            }
        }
    }
    None
}

// Function to print the labyrinth
fn print_labyrinth(labyrinth: &[[bool; N]; N], path: &Vec<(usize, usize)>) {
    let mut labyrinth_with_path = labyrinth.clone();
    for &(x, y) in path {
        labyrinth_with_path[x][y] = false;
    }
    for i in 0..N {
        for j in 0..N {
            if path.contains(&(i, j)) {
                print!("🟩"); // Path
            } else if labyrinth_with_path[i][j] {
                print!("⬛"); // Wall
            } else {
                print!("⬜"); // Empty space
            }
        }
        println!();
    }
}
