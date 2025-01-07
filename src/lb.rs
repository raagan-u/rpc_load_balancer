pub struct LoadBalancer {
    backends: Vec<String>,
    current_index: u32,
}

impl LoadBalancer {
    fn new(self, upstreams: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            backends: upstreams,
            current_index: 0,
        }
    }

    fn check_health(frequency: u32) {
        // logic to check live servers at given frequency
    }

    fn select() -> String {}
}
