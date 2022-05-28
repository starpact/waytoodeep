use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tracing::info;

pub struct DumbFuture {}

impl Future for DumbFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("Hello from a dumb future!");
        Poll::Ready(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Report;

    #[tokio::test]
    async fn test_dump_future() -> Result<(), Report> {
        crate::setup()?;

        info!("Building that dumb future...");
        let fut = DumbFuture {};
        info!("Awaiting that dumb future...");
        fut.await;
        info!("Done awaiting that dumb future");

        Ok(())
    }

    #[test]
    fn a() {
        println!("{}", std::mem::size_of::<Box<dyn std::error::Error>>())
    }
}
