// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(op) = option {
        for x in 0..=op {
            res += x;
        }
    };
}
