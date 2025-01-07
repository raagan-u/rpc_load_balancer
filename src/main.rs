<<<<<<< HEAD
use constants::RPC_URLS;
mod constants;
mod lb;

fn main() {
    env_logger::init();
=======
mod lb;

use lb::LB;
use pingora::prelude::*;

fn main() {
    let mut server = Server::new(None).unwrap();

    let mut upstreams =
        LoadBalancer::try_from_iter(["127.0.0.1:5678", "127.0.0.1:1234", "127.0.0.1:3333"])
            .unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    let mut lb = http_proxy_service(&server.configuration, LB(upstreams));

    lb.add_tcp("0.0.0.0:8080");

    server.add_service(lb);
    server.bootstrap();
    server.run_forever();
>>>>>>> parent of 74b37fe (#2 fix; add health check background service)
}
