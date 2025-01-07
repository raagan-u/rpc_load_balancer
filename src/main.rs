mod constants;
mod lb;

use constants::RPC_URLS;
use lb::LB;
use pingora::prelude::*;

fn main() {
    let mut server = Server::new(None).unwrap();

    let mut upstreams = LoadBalancer::try_from_iter(RPC_URLS).unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams: std::sync::Arc<
        LoadBalancer<
            pingora::lb::selection::weighted::Weighted<
                pingora::lb::selection::algorithms::RoundRobin,
            >,
        >,
    > = background.task();

    let mut lb = http_proxy_service(&server.configuration, LB(upstreams));

    lb.add_tcp("0.0.0.0:8080");

    server.add_service(lb);
    server.add_service(background);

    server.bootstrap();
    server.run_forever();
}

// #[cfg(test)]
// mod test{

//     #[test]

// }
