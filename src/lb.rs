<<<<<<< HEAD
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
=======
use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

pub struct LB(pub Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();

        println!("upstream peer is {:#?}", upstream);

        let peer = Box::new(HttpPeer::new(
            upstream,
            false,
            "one.one.one.one".to_string(),
        ));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }
>>>>>>> parent of 74b37fe (#2 fix; add health check background service)
}
