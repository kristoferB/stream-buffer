//! SPState represents a state in SP
//!

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

/// Representing a State in SP with variables and their values. The values are
/// stored in a vec to speed up reading and writing. The position of a value in the
/// vec is stored in the Hashmap. This should be cashed by user of the state for
/// index access.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SPState {
    index: HashMap<SPPath, usize>,
    values: Vec<StateValue>,
    id: Uuid,
}

// TODO: Maybe check the id?
impl PartialEq for SPState {
    fn eq(&self, other: &Self) -> bool {
        self.projection() == other.projection()
    }
}

#[derive(Debug, Clone)]
pub struct StateProjection<'a> {
    pub projection: Vec<(&'a SPPath, &'a StateValue)>,
    pub id: Uuid,
}

impl PartialEq for StateProjection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.projection == other.projection
    }
}

impl<'a> StateProjection<'a> {
    fn clone_state(&self) -> SPState {
        SPState::new_from_state_values(&self.clone_vec())
    }
    fn clone_vec(&self) -> Vec<(SPPath, StateValue)> {
        self.projection
            .iter()
            .map(|(p, v)| ((*p).clone(), (*v).clone()))
            .collect()
    }
    fn clone_vec_value(&self) -> Vec<(SPPath, SPValue)> {
        self.projection
            .iter()
            .map(|(p, v)| ((*p).clone(), (*v).value().clone()))
            .collect()
    }
    fn sort(&mut self) {
        self.projection
            .sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
    }
}

/// StateValue includes the current and an optional next and prev value.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct StateValue {
    current: SPValue,
    next: Option<SPValue>,
    prev: Option<SPValue>,
}

impl StateValue {
    pub fn new(value: SPValue) -> StateValue {
        StateValue {
            current: value,
            next: None,
            prev: None,
        }
    }
    pub fn next(&mut self, value: SPValue) -> bool {
        if self.next.is_none() {
            self.next = Some(value);
            true
        } else {
            false
        }
    }
    pub fn take(&mut self) -> bool {
        if self.next.is_none() {
            false
        } else {
            let n = self.next.take().unwrap();
            let p = std::mem::replace(&mut self.current, n);
            self.prev = Some(p);
            true
        }
    }
    pub fn force(&mut self, value: SPValue) {
        let p = std::mem::replace(&mut self.current, value);
        self.prev = Some(p);
    }
    pub fn revert_next(&mut self) {
        self.next = None;
    }
    pub fn revert_to_prev(&mut self) {
        if self.prev.is_some() {
            self.current = self.prev.take().unwrap();
            self.next = None; // Maybe we should allow a next, but i think it is better like this
        }
    }
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
    pub fn value(&self) -> &SPValue {
        match self.next {
            Some(ref x) => x,
            None => &self.current
        }
    }
    pub fn current_value(&self) -> &SPValue {
        &self.current
    }
    pub fn next_value(&self) -> &Option<SPValue> {
        &self.next
    }

    pub fn previous_value(&self) -> &Option<SPValue> {
        &self.prev
    }
    pub fn extract(self) -> SPValue {
        match  self.next {
            Some(x) => x,
            None => self.current
        }
    }
}

/// The StatePath is used to speed up the access of variables in the state by instead using the index of the variable 
/// in the state instead of the path. You may have to check that the state has the same id as the state path if
/// you do not have control over the state. However, the methods that take a path will also check it.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StatePath {
    path: SPPath,
    index: usize,
    state_id: Uuid,
}



impl SPState {
    /// Creates a new empty state.
    pub fn new() -> SPState {
        SPState {
            index: HashMap::new(),
            values: Vec::new(),
            id: Uuid::new_v4(),
        }
    }

    /// Creates a new State that includes the state defined in the array tuple. Preferred method
    pub fn new_from_values(hm: &[(SPPath, SPValue)]) -> SPState {
        let mut xs = HashMap::new();
        let mut v = Vec::with_capacity(hm.len());
        for (key, value) in hm.iter() {
            v.push(StateValue::new(value.clone()));
            xs.insert(key.clone(), v.len() - 1);
        }
        SPState {
            index: xs,
            values: v,
            id: Uuid::new_v4(),
        }
    }

    /// Creates a new State that includes the state defined in the hashmap.
    pub fn new_from_state_values(hm: &[(SPPath, StateValue)]) -> SPState {
        let mut xs = HashMap::new();
        let mut v = Vec::with_capacity(hm.len());
        for (key, value) in hm.iter() {
            v.push(value.clone());
            xs.insert(key.clone(), v.len() - 1);
        }
        SPState {
            index: xs,
            values: v,
            id: Uuid::new_v4(),
        }
    }

    pub fn add_state_variable(&mut self, path: SPPath, value: StateValue) {
        if self.index.contains_key(&path) {
            self.values[*self.index.get(&path).unwrap()] = value;
        } else {
            self.values.push(value);
            self.index.insert(path, self.values.len() - 1);
            self.id = Uuid::new_v4(); // the index has changed and it is probably better to reload
        }
    }
    pub fn add_state_variables(&mut self, map: Vec<(SPPath, StateValue)>) {
        map.into_iter().for_each(|(path, value)| {
            self.add_state_variable(path, value);
        });
    }
    

    /// Add a new state variable to the state. If the path already is included, the value is updated.
    /// Maybe we should change this and not update the state? This will also change the id of the state
    pub fn add_variable(&mut self, path: SPPath, value: SPValue) {
        let new_v = StateValue::new(value);
        self.add_state_variable(path, new_v);
    }

    /// Add new variables to the state. If a path is already included, the value is updated.
    /// Maybe we should change this and not update the state?
    pub fn add_variables(&mut self, map: Vec<(SPPath, SPValue)>) {
        map.into_iter().for_each(|(path, value)| {
            self.add_variable(path, value);
        });
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the index of a variable to be used for faster access. Cash this and compare with
    /// the id of the state if it is still valid
    pub fn state_path(&self, path: &SPPath) -> Option<StatePath> {
        self.index.get(path).map(|index| StatePath {
            path: path.clone(),
            index: *index,
            state_id: self.id,
        })
    }

    pub fn check_state_path(&self, state_path: &StatePath) -> bool {
        state_path.state_id == self.id && self.values.len() > state_path.index
    }

    /// The standard way of getting values from the state. Get the state_path first
    pub fn state_value(&self, state_path: &StatePath) -> Option<&StateValue> {
        if self.check_state_path(state_path) {
            Some(&self.values[state_path.index])
        } else {
            self.state_value_from_path(&state_path.path)
        }
    }

    pub fn sp_value(&self, state_path: &StatePath) -> Option<&SPValue> {
        self.state_value(state_path).map(|x| x.current_value())
    }

    /// Get a StateValue from the state based on a path. Only use this when you need do not
    /// need to get the same variable multiple times. Else use state_value_with_index
    pub fn state_value_from_path(&self, path: &SPPath) -> Option<&StateValue> {
        self.index.get(path).map(|i| &self.values[*i])
    }

    /// Get a StateValue based on its index when you need to access the same variable multiple
    /// times. First get and cash the index from fn state_path(&path), and then use that index.
    /// If you do not know if the state can change, check the state id.
    /// This fn will panic if i is larger than no of values
    pub fn state_value_from_index(&self, i: usize) -> &StateValue {
        &self.values[i]
    }

    /// Get a SPValue from the state based on a path. Only use this when you need do not
    /// need to get the same variable multiple times. Else use sp_value
    pub fn sp_value_from_path(&self, path: &SPPath) -> Option<&SPValue> {
        self.state_value_from_path(path).map(|x| x.current_value())
    }

    /// Get a SPValue based on its index when you need to access the same variable multiple
    /// times. First get and cash the index from fn state_path(&path), and then use that index.
    /// If you do not know if the state can change, check the state id.
    /// This fn will panic if i is larger than no of values
    pub fn sp_value_from_index(&self, i: usize) -> &SPValue {
        self.state_value_from_index(i).current_value()
    }

    /// Get a projection of the state
    pub fn projection(&self) -> StateProjection {
        let s: Vec<(&SPPath, &StateValue)> = self
            .index
            .iter()
            .map(|(key, i)| (key, &self.values[*i]))
            .collect();

        let mut p = StateProjection {
            projection: s,
            id: self.id,
        };
        p.sort();  // Maybe not this?
        p
    }

    /// Returns a projection of the sub part of the state where the variables are children to the path
    /// TODO: Maybe move this to struct StateProjection
    ///
    /// ["a", "b"] is a child of ["a"]
    ///
    pub fn sub_state_projection(&self, path: &SPPath) -> StateProjection {
        let s: Vec<(&SPPath, &StateValue)> = self
            .index
            .iter()
            .filter(|(key, _)| key.is_child_of(path))
            .map(|(key, i)| (key, &self.values[*i]))
            .collect();

        StateProjection {
            projection: s,
            id: self.id,
        }
    }

    /// Checks if a sub state is the same as another states sub state given the same path
    /// This is used to check this so we do not need to create a clone with sub_state.
    pub fn is_sub_state_the_same(&self, state: &SPState, path: &SPPath) -> bool {
        self.index
            .iter()
            .filter(|(key, _)| key.is_child_of(path))
            .all(|(key, i)| {
                state
                    .sp_value_from_path(key)
                    .map(|x| x == self.values[*i].current_value())
                    .unwrap_or(false)
            })
    }

    pub fn prefix_paths(&mut self, parent: &SPPath) {
        let mut xs = HashMap::new();
        for (path, i) in self.index.iter() {
            let mut new_p = path.clone();
            new_p.add_parent_path(parent);
            xs.insert(new_p, *i);
        }
        self.index = xs;
        self.id = Uuid::new_v4(); // Changing id since the paths changes
    }

    pub fn next_is_allowed(&self, state_path: &StatePath) -> bool {
        self.check_state_path(state_path) && !self.values[state_path.index].has_next()
    }

    pub fn next(&mut self, state_path: &StatePath, value: SPValue) -> SPResult<()> {
        if !self.check_state_path(state_path) {
            Err(SPError::No("The state path is wrong".to_string()))
        } else if !self.values[state_path.index].next(value){
            Err(SPError::No("The state already have a next value".to_string()))
        } else {
            Ok(())
        }
    }
    pub fn next_from_path(&mut self, path: &SPPath, value: SPValue) -> SPResult<()> {
        if let Some(sp) = self.state_path(&path) {
            self.next(&sp, value)
        } else {
            Err(SPError::No(format! {"Can not find the path: {:?}", path}))
        }
    }
    pub fn force(&mut self, state_path: &StatePath, value: SPValue) -> SPResult<()> {
        if !self.check_state_path(state_path) {
            Err(SPError::No("The state path is wrong".to_string()))
        } else {
            self.values[state_path.index].force(value);
            Ok(())
        }
    }

    
    pub fn next_map(&mut self, map: Vec<(StatePath, SPValue)>) -> bool {
        let ok = map.iter().all(|(p, _)| {
            self.next_is_allowed(p)
        });
        ok && {
            map.into_iter().for_each(|(p, v)| {self.values[p.index].next(v);});
            true
        }
    }

    pub fn take_transition(&mut self) {
        self.values.iter_mut().for_each(|v| {
            v.take();
        });
    }

    pub fn extract(self) -> Vec<(SPPath, StateValue)> {
        let index = self.index;
        let mut values = self.values;
        index
            .into_iter()
            .map(|(key, i)| (key, std::mem::replace(&mut values[i], StateValue::new(SPValue::Unknown))))
            .collect()
    }

    pub fn extend(&mut self, other_state: SPState) {
        let p = other_state.extract();
        self.add_state_variables(p);
    }
}

impl fmt::Display for SPState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Sort keys by name.
        let proj = self.projection();
        let mut buf = Vec::new();
        for (p, val) in proj.projection {
            buf.push(format!("{}: {:?}", p, val));
        }
        write!(f, "{}", buf.join("\n"))
    }
}



/// helping making states with a macro
#[macro_export]
macro_rules! state {

    ($( $key: ident => $val: expr ),*) => {{
        let mut s: Vec<(SPPath, StateValue)> = Vec::new();
        $(
            s.push(($key.clone(), StateValue::new($val.to_spvalue())));
        )*
        SPState::new_from_state_values(&s)
    }};
    ($( $key: expr => $val: expr ),*) => {{
        let mut s: Vec<(SPPath, StateValue)> = Vec::new();
        $(
            let xs: Vec<String> = $key.iter().map(|x|x.to_string()).collect();
            s.push((SPPath::from(xs), StateValue::new($val.to_spvalue())));
        )*
        SPState::new_from_state_values(&s)
    }}
}



/// ********** TESTS ***************

#[cfg(test)]
mod sp_value_test {
    use super::*;
    #[test]
    fn create_state() {
        let ab = SPPath::from_slice(&["a", "b"]);
        let ac = SPPath::from_slice(&["a", "c"]);
        let kl = SPPath::from_slice(&["k", "l"]);
        let s = state!(["a", "b"] => 2, ["a", "c"] => true, ["k", "l"] => true);
        let s2 = state!(ab => 2, ac => true, kl => true);
        println!("s proj {:?}", s.projection());
        println!("s2 proj {:?}", s2.projection());

        assert_eq!(s, s2);

    }

    #[test]
    fn get_value() {
        let s = state!(["a", "b"] => 2, ["a", "c"] => true, ["k", "l"] => true);

        let v = SPPath::from_slice(&["a", "b"]);
        let res = s.sp_value_from_path(&v);

        assert_eq!(res, Some(&2.to_spvalue()));
    }

    // #[test]
    // fn get_substate() {
    //     let s = state!(["a", "b"] => 2, ["a", "c"] => true, ["k", "l"] => true);

    //     let a = &vec!("a".to_string());
    //     let sub = s.filter(a);

    //     assert_eq!(sub, state!(["a", "b"] => 2, ["a", "c"] => true));
    // }

    #[test]
    fn next() {
        let mut s = state!(["a", "b"] => 2, ["a", "c"] => true, ["k", "l"] => true);
        let v = &SPPath::from_slice(&["a", "b"]);
        let state_path = s.state_path(v).unwrap();
        let x = s.next(&state_path, 5.to_spvalue());
        assert_eq!(Ok(()), x);

        let x = s.next(&state_path, 5.to_spvalue());
        assert!(x.is_err());





        println!("{:?}", x);
        println!("{:?}", s);
    }
    #[test]
    fn take_all_next() {
        let mut s = state!(["a", "b"] => 2, ["a", "c"] => true, ["k", "l"] => true);
        let ab = &SPPath::from_slice(&["a", "b"]);
        let ac = &SPPath::from_slice(&["a", "c"]);
        let p_ab = s.state_path(ab).unwrap();
        let p_ac = s.state_path(ac).unwrap();

        // s.next(&p_ab, 5.to_spvalue()).expect("Oh no");
        // s.next(&p_ac, AssignStateValue::Delay(false.to_spvalue(), 1000))
        //     .expect("oh no");
        // println!("The state before: {:?}", s);
        // s.take_transition();

        // assert_eq!(
        //     s.state_value(&p_ac),
        //     Some(&StateValue::Delay(Delay {
        //         current_value: true.to_spvalue(),
        //         next_value: false.to_spvalue(),
        //         millis: 1000,
        //         has_been_spawned: true
        //     }))
        // );
        // assert_eq!(
        //     s.state_value(&p_ab),
        //     Some(&StateValue::SPValue(5.to_spvalue()))
        // );

        // println!("The state: {:?}", s);
    }

    // #[test]
    // fn sub_state_testing() {
    //     let a = SPPath::from_slice(&["a"]);
    //     let ab = SPPath::from_array(&["a", "b"]);
    //     let ax = SPPath::from_array(&["a", "x"]);
    //     let abc = SPPath::from_array(&["a", "b", "c"]);
    //     let abx = SPPath::from_array(&["a", "b", "x"]);
    //     let b = SPPath::from_array(&["b"]);
    //     let s = state!(abc => false, abx => false, ax => true);

    //     assert_eq!(s.sub_state(&ab), state!(abc => false, abx => false));
    //     assert_eq!(s.sub_state(&abc), state!(abc => false));
    //     assert_eq!(s.sub_state(&b), state!());
    //     assert_eq!(s.sub_state(&a), s.clone());

    //     let mut s2 = s.clone();
    //     assert!(s.is_sub_state_the_same(&s2, &ab));
    //     s2.insert(&ax, false.to_spvalue()
    //        .unwrap();
    //     assert!(s.is_sub_state_the_same(&s2, &ab));
    //     s2.insert(&abc, true.to_spvalue()
    //        .unwrap();
    //     assert!(!s.is_sub_state_the_same(&s2, &ab));
    // }
}
