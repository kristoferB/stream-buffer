# stream-buffer

ChannelMerger is a stream buffer that will fold messages waiting in 
the incoming channel. Implement a folding function that merges the 
items and return true if an item was merged with prvious. 
If you do not want to merge two items, just return false.alloc

# Example
```
let (tx, rx) = channel(2);
let mut merger = ChannelMerger::new(
    rx,
    |x: &mut usize, y: &usize| {  // merging until we get a 0
        *x += y;
        *x != 0
    }
);

task::block_on(async move {
while let Some(x) = merger.next().await {
   std::println!("GOT1: {:?}", x);
}
});

```
