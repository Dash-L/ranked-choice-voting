use std::path::Path;

use compute_rcv::count_rcv;
fn main() {
    count_rcv(Path::new("better_vote_data.mpack"));
}
