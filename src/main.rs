use async_std::*;
use prelude::*;
use task;
use sync::channel;
use sync::{Receiver};
use stream::Stream;
use pin::Pin;
use pin_project_lite::pin_project;


fn main() {
    let (tx, rx) = channel(2);
    let mut merger = ChannelMerger::new(
        rx,
        |x: &mut usize, y: &usize| {
            *x += y;
            *x != 0
            //return false;
        }
    );



    task::spawn( async move { 
        for x in 1..10 {
            tx.send(x).await
        }
        tx.send(0).await;
        tx.send(2).await;
        tx.send(3).await;
        tx.send(0).await;
        tx.send(5).await;
        tx.send(6).await;
        std::println!("Everything is sent");
    });

    task::block_on(async move {
        while let Some(x) = merger.next().await {
            std::println!("GOT1: {:?}", x);
        }
    });
}

pin_project! {
    struct ChannelMerger<T, F> where F: FnMut(&mut T, &T) -> bool {
        #[pin] rx: Receiver<T>,
        merger: F, 
        buffer: Option<T>,
    }
}

impl<T, F> ChannelMerger<T, F> where F: FnMut(&mut T, &T) -> bool {
    fn new(rx: Receiver<T>, merger: F) -> ChannelMerger<T, F> {
        ChannelMerger{
            rx, 
            merger,
            buffer: None,
        }
    }
}

impl<T, F> Stream for ChannelMerger<T, F>  where T: Clone, F: FnMut(&mut T, &T) -> bool {
    type Item = T;

    // poll_next() is the only required method
    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<Self::Item>> {
        let this = self.project();
        loop {
            if this.rx.is_empty() && this.buffer.is_none() {
                return this.rx.poll_next(cx);
            } else if this.rx.is_empty() && this.buffer.is_some() {
                let temp = this.buffer.clone();
                *this.buffer = None;
                return task::Poll::Ready(temp);
            } else {
                let x = task::block_on(this.rx.recv()).unwrap();
                if this.buffer.is_none() {
                    *this.buffer = Some(x);
                } else if !(this.merger)(this.buffer.as_mut().unwrap(), &x) {
                    let temp = this.buffer.clone();
                    *this.buffer = Some(x);
                    return task::Poll::Ready(temp);
                }

            }
        }
        
    }
}