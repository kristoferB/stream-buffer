use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("hashmap state", |b| b.iter(|| black_box(eval_hash())));
        c.bench_function("vec state", |b| b.iter(|| black_box(eval_vec())));
 

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


fn make_hashmap(size: usize) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    for i in 0 .. size {
        hm.insert(i.to_string(), i.to_string());
    }
    hm
}
fn make_vec(size: usize) -> Vec<String> {
    let mut hm = Vec::with_capacity(size);
    for i in 0 .. size {
        hm.push(i.to_string());
    }
    hm
}


fn make_random() -> [String; 10] {
    [
        "0".to_string(), 
        "10".to_string(),
        "20".to_string(),
        "30".to_string(),
        "40".to_string(),
        "55".to_string(),
        "60".to_string(),
        "70".to_string(),
        "80".to_string(),
        "90".to_string(),
    ]
}
fn make_random_int() -> [usize; 10] {
    [
        0, 
        10,
        20,
        30,
        40,
        55,
        60,
        70,
        80,
        90,
    ]
}


#[allow(clippy::logic_bug)]
fn eval_hash() -> usize {
    let mut state = make_hashmap(100);
    let r = make_random();
    let mut k = 0;

    for i in 0 .. 1000 {
        let ev1 = state.get(&r[0]).unwrap() == "0" && 
        state.get(&r[3]).unwrap() == "30" && 
        !(state.get(&r[4]).unwrap() == "40") || 
        state.get(&r[5]).unwrap() == "50" && 
        state.get(&r[2]).unwrap() == "20" && 
        state.get(&r[6]).unwrap() == "60" || 
        state.get(&r[7]).unwrap() == "70" && 
        state.get(&r[8]).unwrap() == "80" || 
        state.get(&r[3]).unwrap() == "30";

        if ev1 || !ev1 {
            upd_s(&mut state, &r[6]);
            upd_s(&mut state, &r[5]);
            upd_s(&mut state, &r[2]);
            upd_s(&mut state, &r[1]);
            upd_s(&mut state, &r[7]);
            upd_s(&mut state, &r[8]);
        }

        k = i;
    }

    k 
}
#[allow(clippy::logic_bug)]
fn eval_vec() -> usize {
    let mut state = make_vec(100);
    let r = make_random_int();
    let mut k = 0;

    for i in 0 .. 1000 {
        let ev1 = state[r[0]] == "0" && 
        state[r[3]] == "30" && 
        !(state[r[4]] == "40") || 
        state[r[5]] == "50" && 
        state[r[2]] == "20" && 
        state[r[6]] == "60" || 
        state[r[7]] == "70" && 
        state[r[8]] == "80" || 
        state[r[3]] == "30";

        // if ev1 || !ev1 {
        //     state[r[1]] = r[6].to_string();
        //     state[r[2]] = r[5].to_string();
        //     state[r[3]] = r[2].to_string();
        //     state[r[4]] = r[1].to_string();
        //     state[r[5]] = r[7].to_string();
        //     state[r[6]] = r[8].to_string();
        // }

        k = i;
    }

    k 
}

fn upd_s(state: &mut HashMap<String, String>, k: &str) {
    let x = state.entry(k.to_string()).or_insert_with(|| k.to_string());
    *x = k.to_string();
}

