fn main() {
    explore_con();
}

pub fn explore_con() {
    u07::threaded::waiting_on_workers_with_join_handles();
    u07::threaded::waiting_on_workers_with_channels();
}
