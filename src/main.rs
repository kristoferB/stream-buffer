use async_std::*;
use prelude::*;
use task;
use sync::channel;
use sync::{Receiver};
use stream::Stream;
use pin::Pin;
use pin_project_lite::pin_project;


fn main() {
    let (tx, rx) = channel(10);
    let mut merger = ChannelMerger::new(
        rx,
        |x: &mut usize, y: &usize| {  // merging until we get a 0
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
            std::println!("GOT: {:?}", x);
        }
    });
}

pin_project! {
    /// ChannelMerger is a stream buffer that will fold messages waiting in 
    /// the incoming channel until the channel is empty or if the merge fn
    /// returns false. This will happen for each call to next. 
    /// 
    /// Implement a folding function that merges the 
    /// items and return true if an item was merged with previous. 
    /// If you do not want to merge two items, just return false
    /// 
    /// # Example
    /// ```
    /// let (tx, rx) = channel(2);
    /// let mut merger = ChannelMerger::new(
    ///     rx,
    ///     |x: &mut usize, y: &usize| {  // merging until we get a 0
    ///         *x += y;
    ///         *x != 0
    ///     }
    /// );
    /// 
    /// task::block_on(async move {
    /// while let Some(x) = merger.next().await {
    ///    std::println!("GOT: {:?}", x);
    /// }
    /// });
    /// 
    /// ```
    ///
    pub struct ChannelMerger<T, F> where F: FnMut(&mut T, &T) -> bool {
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