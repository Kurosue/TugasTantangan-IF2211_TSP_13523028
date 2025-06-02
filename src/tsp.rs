use std::collections::HashMap;
use std::io::{self, BufRead};

type MemoKey = (usize, u32);
type MemoVal = (usize, Option<usize>); // (biaya minimum, kota selanjutnya)

fn tsp_recursive(
    current: usize,
    visited: u32,
    n: usize,
    dist: &Vec<Vec<usize>>,
    memo: &mut HashMap<MemoKey, MemoVal>,
) -> usize {
    // Jika S = ∅, maka f(i, ∅) = c[i][1] (kembali langsung ke simpul awal)
    if visited == 0 {
        return dist[current][0]; // cᵢ₁
    }

    // Jika sudah dihitung sebelumnya, langsung ambil
    if let Some(&(cost, _)) = memo.get(&(current, visited)) {
        return cost;
    }

    let mut min_cost = usize::MAX;
    let mut next_city = None;

    // f(i, S) = min { c_i_j + f(j, S - {j}) }
    for next in 1..n {
        let bit = 1 << (next - 1);
        if visited & bit != 0 {
            let remaining = visited ^ bit; // S - {j}
            let cost = dist[current][next] + tsp_recursive(next, remaining, n, dist, memo);
            if cost < min_cost {
                min_cost = cost;
                next_city = Some(next);
            }
        }
    }

    // Simpan hasil f(i, S)
    memo.insert((current, visited), (min_cost, next_city));
    min_cost
}


// Helper buat rekonstruksi path nya biar bisa dilihat
fn reconstruct_path(
    current: usize,
    visited: u32,
    memo: &HashMap<MemoKey, MemoVal>,
    path: &mut Vec<usize>,
) {
    if let Some(&(_, Some(next))) = memo.get(&(current, visited)) {
        path.push(next);
        let bit = 1 << (next - 1);
        reconstruct_path(next, visited ^ bit, memo, path);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input jumlah kota
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Input matriks jarak antar kota ato weight graphnya
    let mut dist = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        dist.push(row);
    }
    // Bitmask dari semua kota kecuali kota 1 (indeks 0)
    let mut visited_mask = 0;
    for i in 1..n {
        visited_mask |= 1 << (i - 1);
    }

    let mut memo: HashMap<MemoKey, MemoVal> = HashMap::new();

    // Hitung f(1, V − {1}) => hasil akhir DP (cost tur minimum)
    let cost = tsp_recursive(0, visited_mask, n, &dist, &mut memo);

    // Konstruksi kembali path dari hasil memo
    let mut path = vec![0]; // mulai dari kota 1 (indeks 0)
    reconstruct_path(0, visited_mask, &memo, &mut path);
    path.push(0); // kembali ke awal

    println!("Bobot tur minimum dari simpul 1 adalah: {}", cost);
    print!("Path yang diambil: ");
    for (i, kota) in path.iter().enumerate() {
        print!("{}", kota + 1);
        if i < path.len() - 1 {
            print!(" → ");
        }
    }
    println!();
}

