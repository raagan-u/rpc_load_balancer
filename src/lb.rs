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
            "sepolia.drpc.org".to_string(),
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
            .insert_header("Host", "sepolia.drpc.org")
            .unwrap();
        Ok(())
    }
}
