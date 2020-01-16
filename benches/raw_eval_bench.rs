use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("hashmap state", |b| b.iter(|| black_box(eval_hash_state())));
        c.bench_function("SPstate index", |b| b.iter(|| black_box(eval_SPState_index())));
        c.bench_function("SPstate path", |b| b.iter(|| black_box(eval_SPState_path())));
        //c.bench_function("SPstate old", |b| b.iter(|| black_box(eval_OLDSPState())));
        //c.bench_function("vec state", |b| b.iter(|| black_box(eval_vec())));
        //c.bench_function("hashmap vec state", |b| b.iter(|| black_box(eval_hash_vec())));
 

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
fn make_hashmap_vec(size: usize) -> (HashMap<String, usize>, Vec<String>) {
    let mut v = Vec::with_capacity(size);
    let mut hm = HashMap::new();
    for i in 0 .. size {
        v.push(i.to_string());
        hm.insert(i.to_string(), i);
    }
    (hm, v)
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
fn make_random_path() -> [SPPath; 10] {
    [
       SPPath::from_slice(&["0".to_string()]),
        SPPath::from_slice(&["10".to_string()]),
        SPPath::from_slice(&["20".to_string()]),
        SPPath::from_slice(&["30".to_string()]),
        SPPath::from_slice(&["40".to_string()]),
        SPPath::from_slice(&["55".to_string()]),
        SPPath::from_slice(&["60".to_string()]),
        SPPath::from_slice(&["70".to_string()]),
        SPPath::from_slice(&["80".to_string()]),
        SPPath::from_slice(&["90".to_string()]),
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
#[allow(clippy::logic_bug)]
fn eval_hash_vec() -> usize {
    let (state, mut v) = make_hashmap_vec(100);
    let r = make_random();
    let mut k = 0;

    for i in 0 .. 1000 {
        let ev1 = get_v(&r[0], &state, &v) == "0" && 
        get_v(&r[3], &state, &v) == "30" && 
        !(get_v(&r[4], &state, &v) == "40") || 
        get_v(&r[5], &state, &v) == "50" && 
        get_v(&r[2], &state, &v) == "20" && 
        get_v(&r[6], &state, &v) == "60" || 
        get_v(&r[7], &state, &v) == "70" && 
        get_v(&r[8], &state, &v) == "80" || 
        get_v(&r[3], &state, &v) == "30";

        // if ev1 || !ev1 {
        //     upd_v(&r[6], &r[1], &state, &mut v);
        //     upd_v(&r[5], &r[6], &state, &mut v);
        //     upd_v(&r[2], &r[5], &state, &mut v);
        //     upd_v(&r[1], &r[2], &state, &mut v);
        //     upd_v(&r[7], &r[1], &state, &mut v);
        //     upd_v(&r[8], &r[7], &state, &mut v);
        // }

        k = i;
    }

    k 
}

fn get_v<'a>(k: &str, s: &HashMap<String, usize>, v: &'a Vec<String>) -> &'a str {
    &v[*s.get(k).unwrap()]
}
fn upd_v(k: &str, value: &str, s: &HashMap<String, usize>, v: &mut Vec<String>){
    v[*s.get(k).unwrap()] = value.to_string();
}

fn upd_s(state: &mut HashMap<String, String>, k: &str) {
    let x = state.entry(k.to_string()).or_insert_with(|| k.to_string());
    *x = k.to_string();
}
fn upd_state(state: &mut HashMap<SPPath, SPValue>, p: SPPath, v: SPValue) {
    let x = state.entry(p).or_insert_with(|| v.clone());
    *x = v;
}

use spbench::*;


#[allow(clippy::logic_bug)]
fn eval_SPState_index() -> usize {
    let mut state = make_state(100);
    let r = make_random_path();
    let mut k = 0;
    let p0 = state.state_path(&r[0]).unwrap();
    let p1 = state.state_path(&r[1]).unwrap();
    let p2 = state.state_path(&r[2]).unwrap();
    let p3 = state.state_path(&r[3]).unwrap();
    let p4 = state.state_path(&r[4]).unwrap();
    let p5 = state.state_path(&r[5]).unwrap();
    let p6 = state.state_path(&r[6]).unwrap();
    let p7 = state.state_path(&r[7]).unwrap();
    let p8 = state.state_path(&r[8]).unwrap();
    let p9 = state.state_path(&r[9]).unwrap();

    for i in 0 .. 1000 {
        let ev1 = state.sp_value(&p0).unwrap() == &"0".to_spvalue() && 
        state.sp_value(&p3).unwrap() == &"30".to_spvalue() && 
        !(state.sp_value(&p4).unwrap() == &"40".to_spvalue()) || 
        state.sp_value(&p5).unwrap() == &"50".to_spvalue() && 
        state.sp_value(&p2).unwrap() == &"20".to_spvalue() && 
        state.sp_value(&p6).unwrap() == &"60".to_spvalue() || 
        state.sp_value(&p7).unwrap() == &"70".to_spvalue() && 
        state.sp_value(&p8).unwrap() == &"80".to_spvalue() || 
        state.sp_value(&p3).unwrap() == &"30".to_spvalue();

        if ev1 || !ev1 {
            state.next(&p6, (state.sp_value(&p1).unwrap().clone())).unwrap();
            state.next(&p5, (state.sp_value(&p6).unwrap().clone())).unwrap();
            state.next(&p2, (state.sp_value(&p5).unwrap().clone())).unwrap();
            state.next(&p1, (state.sp_value(&p2).unwrap().clone())).unwrap();
            state.next(&p7, (state.sp_value(&p1).unwrap().clone())).unwrap();
            state.next(&p8, (state.sp_value(&p7).unwrap().clone())).unwrap();
        }

        state.take_transition();

        k = i;
    }

    k 
}

#[allow(clippy::logic_bug)]
fn eval_SPState_path() -> usize {
    let mut state = make_state(100);
    let r = make_random_path();
    let mut k = 0;

    for i in 0 .. 1000 {
        let ev1 = state.sp_value_from_path(&r[0]).unwrap() == &"0".to_spvalue() && 
        state.sp_value_from_path(&r[3]).unwrap() == &"30".to_spvalue() && 
        !(state.sp_value_from_path(&r[4]).unwrap() == &"40".to_spvalue()) || 
        state.sp_value_from_path(&r[5]).unwrap() == &"50".to_spvalue() && 
        state.sp_value_from_path(&r[2]).unwrap() == &"20".to_spvalue() && 
        state.sp_value_from_path(&r[6]).unwrap() == &"60".to_spvalue() || 
        state.sp_value_from_path(&r[7]).unwrap() == &"70".to_spvalue() && 
        state.sp_value_from_path(&r[8]).unwrap() == &"80".to_spvalue() || 
        state.sp_value_from_path(&r[3]).unwrap() == &"30".to_spvalue();

        if ev1 || !ev1 {
            state.next_from_path(&r[6], (state.sp_value_from_path(&r[1]).unwrap().clone())).unwrap();
            state.next_from_path(&r[5], (state.sp_value_from_path(&r[6]).unwrap().clone())).unwrap();
            state.next_from_path(&r[2], (state.sp_value_from_path(&r[5]).unwrap().clone())).unwrap();
            state.next_from_path(&r[1], (state.sp_value_from_path(&r[2]).unwrap().clone())).unwrap();
            state.next_from_path(&r[7], (state.sp_value_from_path(&r[1]).unwrap().clone())).unwrap();
            state.next_from_path(&r[8], (state.sp_value_from_path(&r[7]).unwrap().clone())).unwrap();
        }

        state.take_transition();

        k = i;
    }

    k 
}


fn make_hashmap_state(size: usize) -> HashMap<SPPath, SPValue> {
    let mut hm = HashMap::new();
    for i in 0 .. size {
        let name = i.to_string();
        let value = name.to_spvalue();
        let path = SPPath::from_slice(&[name]);
        hm.insert(path, value);
    }
    hm
}

#[allow(clippy::logic_bug)]
fn eval_hash_state() -> usize {
    let mut state = make_hashmap_state(100);
    let r = make_random_path();
    let mut k = 0;

    for i in 0 .. 1000 {
        let ev1 = state.get(&r[0]).unwrap() == &"0".to_spvalue() && 
        state.get(&r[3]).unwrap() == &"30".to_spvalue() && 
        !(state.get(&r[4]).unwrap() == &"40".to_spvalue()) || 
        state.get(&r[5]).unwrap() == &"50".to_spvalue() && 
        state.get(&r[2]).unwrap() == &"20".to_spvalue() && 
        state.get(&r[6]).unwrap() == &"60".to_spvalue() || 
        state.get(&r[7]).unwrap() == &"70".to_spvalue() && 
        state.get(&r[8]).unwrap() == &"80".to_spvalue() || 
        state.get(&r[3]).unwrap() == &"30".to_spvalue();

        if ev1 || !ev1 {
            let r4 = state.get(&r[4]).unwrap().clone();
            let r2 = state.get(&r[2]).unwrap().clone();
            let r6 = state.get(&r[6]).unwrap().clone();
            let r7 = state.get(&r[7]).unwrap().clone();
            let r9 = state.get(&r[9]).unwrap().clone();
            let r1 = state.get(&r[1]).unwrap().clone();
            upd_state(&mut state, r[6].clone(), r4);
            upd_state(&mut state, r[5].clone(), r2);
            upd_state(&mut state, r[2].clone(), r6);
            upd_state(&mut state, r[1].clone(), r7);
            upd_state(&mut state, r[7].clone(), r9);
            upd_state(&mut state, r[8].clone(), r1);
        }

        k = i;
    }

    k 
}


fn make_state(size: usize) -> SPState {
    let mut v = Vec::with_capacity(size);
    for i in 0 .. size {
        let name = i.to_string();
        let value = name.to_spvalue();
        let path = SPPath::from_slice(&[name]);
        v.push((path, value));
    }
    SPState::new_from_values(&v)
}


