use rand::Rng;
use std::collections::VecDeque;

// Constants for the size of the labyrinth and the maximum number of steps
const N: usize = 16;
const MAX_STEPS: usize = 64;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    let mut total = 0;
    let mut solvable = 0;
    // Generate a million random labyrinths
    for _ in 0..1000000 {
        let mut labyrinth = [[false; N]; N];
        // Fill the labyrinth with random walls
        for i in 0..N {
            for j in 0..N {
                if i == 0 && j == 0 {
                    labyrinth[i][j] = false; // Ensure start is a path
                } else {
                    labyrinth[i][j] = rng.gen(); // Randomly generate a wall or path
                }
            }
        }
        total += 1;
        // Check if the labyrinth is solvable
        if is_solvable(&labyrinth) {
            solvable += 1;
        }
    }
    // Print the probability of a labyrinth being solvable
    println!("Probability: {}", solvable as f64 / total as f64);
}

// Function to check if a labyrinth is solvable
fn is_solvable(labyrinth: &[[bool; N]; N]) -> bool {
    let mut visited = [[false; N]; N];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((x, y, steps)) = queue.pop_front() {
        // If we've reached the end, the labyrinth is solvable
        if x == N - 1 && y == N - 1 {
            return true;
        }
        // Check all four possible directions
        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            // Check if the new position is within the labyrinth and is not a wall
            if nx >= 0 && nx < N as i32 && ny >= 0 && ny < N as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !labyrinth[nx][ny] && !visited[nx][ny] && steps + 1 <= MAX_STEPS {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny, steps + 1));
                }
            }
        }
    }
    // If we've exhausted all possibilities without reaching the end, the labyrinth is not solvable
    false
}
