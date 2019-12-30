use async_std::*;
use prelude::*;
use task;
use sync::channel;

use std::collections::hash_map::HashMap;

mod stream_buffer;
use stream_buffer::StreamBuffer;


fn main() {
    // Simple example when sending usize and adding when item is not 0
    let (tx, rx) = channel(2);
    let mut merger = StreamBuffer::new(
        rx,
        |x: &mut usize, y: &usize| {  // merging until we get a 0
            *x += y;
            *x != 0
            //return false;
        }
    );

    // Example when struct has a hashmap that will always merge
    let (tx2, rx2) = channel(100);
    let mut merger2 = StreamBuffer::new(
        rx2,
        |x: &mut MyState, y: &MyState| {  // merging until we get a 0
            x.xs.extend(y.xs.iter().map(|(x, y)| (x.clone(), y.clone())));
            true
        }
    );

    // Sening usize
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
        std::println!("usize is sent");
    });


    // sending MyState
    task::spawn( async move { 
        for x in 1..1000 {
            let mut m = HashMap::new();
            m.insert(x.to_string(), "hej".to_string());
            let state = MyState{xs: m};
            tx2.send(state).await
        }
        std::println!("MyState is sent");
    });

    // pulling states from buffer
    let res2 = task::spawn(async move {
        while let Some(x) = merger2.next().await {
            std::println!("GOT: {:?}", x);
        }
    });

    // pulling usize from buffer
    let res = task::spawn(async move {
        while let Some(x) = merger.next().await {
            std::println!("GOT: {:?}", x);
        }
    });



    task::block_on(async move {
        res.await;
        res2.await;
    })

}

#[derive(Clone, Debug)]
struct MyState {
    xs: std::collections::hash_map::HashMap<String, String>
}


