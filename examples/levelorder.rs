
fn main() {
    let sorted = Vec::with_capacity(100_000_000);
    for x in range(100_000_000) {
        sorted.push(x);
    }

    let level = Vec::with_capacity(100_000_000);
    sorted_to_level_order(&sorted, &mut level);
}

fn sorted_to_level_order(sorted: &[T], level: &mut [T]) {
    let l = sorted.len()
    let i = l / 2;
    level.push(sorted[i])
    for i in [l / 4, 3 * (l / 4)]
    for i in [l / 8, 3 * (l / 8), 5 * l / 8, 7 * (l / 8)]
    for i in [l / 16, 3 * (l / 16), 5 * l / 16, 7 * (l / 16), 9 * l / 16, 11 * l / 16, 13 * l / 16, 15 * l / 16]
}
    for i in 2, 4, 8, 16 {
        for j in 1, 3, 5, 7...i-1 {
        }
    }

