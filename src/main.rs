#![feature(explicit_tail_calls)]
#![expect(incomplete_features)]
// use dev_playground_rust::seed::seed;
use dev_playground_rust::seed_recursive::seed_recursive;

fn main() {
    // seed::run();
    seed_recursive::run();

}
