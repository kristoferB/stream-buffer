// //! OLDSPState represents a state in SP
// //!

// use super::*;
// use serde::{Deserialize, Serialize};
// use std::collections::{HashMap,BTreeMap};
// use std::fmt;

// /// Representing a State in SP with variables and their values. This is used by the runner
// /// and predicates and actions. This should not be sent out, instead use State
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
// pub struct OLDSPState {
//     pub s: HashMap<SPPath, OLDStateValue>,
// }

// /// Representing a State in SP that is shared to others and is used for sending state
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
// pub struct OLDStateExternal {
//     pub s: HashMap<SPPath, SPValue>,
// }

// /// Representing variables that should be assigned to a OLDSPState. Just a wrapper to simplify life
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
// pub struct OLDAssignState {
//     pub s: HashMap<SPPath, OLDAssignStateValue>,
// }

// /// OLDStateValue wrapps the value of a variable in a state. SPValue are the normal type
// /// and OLDDelay and OLDnext are mainly used in the runner.
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub enum OLDStateValue {
//     SPValue(SPValue),
//     OLDDelay(OLDDelay),
//     OLDNext(OLDNext),
//     Unknown,
// }

// /// OLDAssignStateValue is used when assigning a new value to the state
// /// It will either result in a OLDNext or a OLDdelay
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub enum OLDAssignStateValue {
//     SPValue(SPValue),
//     OLDDelay(SPValue, u64),
//     CancelOLDDelay,    // to cancel a OLDDelay
//     Force(SPValue), // used to overwrite OLDNext and OLDDelay StateValues
// }

// pub trait OLDToStateValue {
//     fn to_oldstate(&self) -> OLDStateValue;
// }

// /// OLDDelaying a OLDnext value change in actions. This will be included in the state
// /// and after the OLDdelay, the OLDDelay will be replaced by the new_value in the state.
// ///
// /// Use the OLDDelay action in the action to tell the runner to OLDdelay the change. The
// /// Runner will create a future that can be canceled
// ///
// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub struct OLDDelay {
//     pub current_value: SPValue,
//     pub OLDnext_value: SPValue,
//     pub millis: u64,
//     pub has_been_spawned: bool,
// }

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub struct OLDNext {
//     pub current_value: SPValue,
//     pub OLDnext_value: SPValue,
// }

// impl OLDSPState {
//     // pub fn filter(&self, partial_name: &[String]) -> OLDSPState {
//     //     let s = self.s.iter().filter(|(k, _)| {
//     //         partial_name.iter().all(|x|{
//     //             k.path.contains(x)
//     //         })
//     //     })
//     //     .map(|(k, v)|{ (k.clone(), v.clone())})
//     //     .collect();
//     //     OLDSPState{s}
//     // }

//     pub fn get_value(&self, var: &SPPath) -> Option<&SPValue> {
//         self.s.get(var).map(|x| x.get_value())
//     }
//     pub fn get(&self, var: &SPPath) -> Option<&OLDStateValue> {
//         self.s.get(var)
//     }

//     /// Extract a clone of the sub part of the state where the variables are children to the path
//     ///
//     /// ["a", "b"] is a child of ["a"]
//     ///
//     pub fn sub_state(&self, path: &SPPath) -> OLDSPState {
//         let s = self
//             .s
//             .iter()
//             .filter(|(key, _)| key.is_child_of(path))
//             .map(|(key, value)| (key.clone(), value.clone()))
//             .collect();

//         OLDSPState { s }
//     }

//     /// Checks if a sub state is the same as another states sub state given the same path
//     /// This is used to check this so we do not need to create a clone with sub_state.
//     pub fn is_sub_state_the_same(&self, state: &OLDSPState, path: &SPPath) -> bool {
//         self.s
//             .iter()
//             .filter(|(key, _)| key.is_child_of(path))
//             .all(|(key, value)| state.get(key).map(|x| x == value).unwrap_or(false))
//     }

//     pub fn external(&self) -> OLDStateExternal {
//         let res = self
//             .s
//             .iter()
//             .map(|(key, value)| (key.clone(), value.get_value().clone()))
//             .collect();

//         OLDStateExternal { s: res }
//     }

//     fn make_insert(OLDnext: OLDAssignStateValue, prev: OLDStateValue) -> SPResult<OLDStateValue> {
//         match (OLDnext, prev) {
//             (OLDAssignStateValue::Force(n), _) => Ok(OLDStateValue::SPValue(n)),
//             (OLDAssignStateValue::CancelOLDDelay, OLDStateValue::OLDDelay(p)) => {
//                 Ok(OLDStateValue::SPValue(p.current_value))
//             }
//             (x, OLDStateValue::OLDNext(p)) => {
//                 eprintln!(
//                     "Your are trying to overwrite a OLDnext: current: {:?}, new: {:?} ",
//                     x, p
//                 );
//                 Err(SPError::No(format!{"{:?}-{:?}", x, p}))
//             }
//             (x, OLDStateValue::OLDDelay(p)) => {
//                 eprintln!(
//                     "Your are trying to overwrite a OLDdelay: current: {:?}, new: {:?} ",
//                     x, p
//                 );
//                 Err(SPError::No(format!{"{:?}-{:?}", x, p}))
//             }
//             (OLDAssignStateValue::SPValue(n), OLDStateValue::SPValue(p)) => Ok(OLDStateValue::OLDNext(OLDNext {
//                 current_value: p,
//                 OLDnext_value: n,
//             })),
//             (OLDAssignStateValue::OLDDelay(n, ms), OLDStateValue::SPValue(p)) => {
//                 Ok(OLDStateValue::OLDDelay(OLDDelay {
//                     current_value: p,
//                     OLDnext_value: n,
//                     millis: ms,
//                     has_been_spawned: false,
//                 }))
//             }
//             (OLDAssignStateValue::CancelOLDDelay, OLDStateValue::SPValue(p)) => Ok(OLDStateValue::SPValue(p)),
//             (OLDAssignStateValue::SPValue(n), OLDStateValue::Unknown) => Ok(OLDStateValue::SPValue(n)),
//             (OLDAssignStateValue::OLDDelay(n, _), OLDStateValue::Unknown) => {
//                 // Can not OLDdelay if current is unknown
//                 Ok(OLDStateValue::SPValue(n))
//             }
//             (OLDAssignStateValue::CancelOLDDelay, OLDStateValue::Unknown) => {
//                 eprintln!("Your are trying to cancel an unknown");
//                 Err(SPError::No(
//                     "Your are trying to cancel an unknown".to_string(),
//                 ))
//             }
//         }
//     }

//     pub fn insert(&mut self, key: &SPPath, value: OLDAssignStateValue) -> SPResult<()> {
//         let x = self.s.entry(key.clone()).or_insert(OLDStateValue::Unknown);
//         match OLDSPState::make_insert(value, x.clone()) {
//             Ok(v) => {
//                 *x = v;
//                 Ok(())
//             }
//             Err(e) => Err(e),
//         }
//     }

//     pub fn insert_map(&mut self, map: OLDAssignState) -> SPResult<()> {
//         for (var, value) in map.s.into_iter() {
//             self.insert(&var, value)?
//         }
//         Ok(())
//     }

//     pub fn take_all_OLDnext(&mut self) {
//         self.s.iter_mut().for_each(|(_, v)| {
//             if let OLDStateValue::OLDNext(n) = v {
//                 *v = OLDStateValue::SPValue(n.OLDnext_value.clone());
//             } else if let OLDStateValue::OLDDelay(d) = v {
//                 d.has_been_spawned = true;
//             }
//         });
//     }

//     pub fn is_allowed(&self, key: &SPPath, value: OLDAssignStateValue) -> bool {
//         self.s
//             .get(key)
//             .map(|x| OLDSPState::make_insert(value, x.clone()).is_ok())
//             .unwrap_or(false)
//     }

//     pub fn extend(&mut self, other_state: OLDSPState) {
//         self.s.extend(other_state.s)
//     }
// }

// impl OLDStateExternal {
//     pub fn new() -> OLDStateExternal {
//         OLDStateExternal { s: HashMap::new() }
//     }

//     pub fn prefix_paths(&self, parent: &GlobalPath) -> OLDStateExternal {
//         let s = self.s.iter().map(|(k,v)| {
//             if let SPPath::LocalPath(lp) = k {
//                 (lp.to_global(parent).to_sp(), v.clone())
//             } else {
//                 (k.clone(), v.clone())
//             }
//         }).collect();
//         OLDStateExternal { s }
//     }

//     pub fn unprefix_paths(&self, parent: &GlobalPath) -> OLDStateExternal {
//         let s = self.s.iter().map(|(k,v)| {
//             if let SPPath::GlobalPath(gp) = k {
//                 (gp.to_local(parent).to_sp(), v.clone())
//             } else {
//                 (k.clone(), v.clone())
//             }
//         }).collect();
//         OLDStateExternal { s }
//     }

//     pub fn to_OLDspstate(&self) -> OLDSPState {
//         let res = self
//             .s
//             .iter()
//             .map(|(key, value)| (key.clone(), OLDStateValue::SPValue(value.clone())))
//             .collect();

//         OLDSPState { s: res }
//     }

//     pub fn to_OLDassignstate(&self) -> OLDAssignState {
//         let res = self
//             .s
//             .iter()
//             .map(|(key, value)| (key.clone(), OLDAssignStateValue::SPValue(value.clone())))
//             .collect();

//         OLDAssignState { s: res }
//     }
// }

// impl fmt::Display for OLDStateExternal {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Sort keys by name.
//         let ordered: BTreeMap<_, _> = self.s.iter().collect();
//         let mut buf = Vec::new();
//         for (p, val) in &ordered {
//             buf.push(format!("{}: {:?}", p, val));
//         }
//         write!(f, "{}", buf.join("\n"))
//     }
// }

// impl OLDStateValue {
//     fn get_value(&self) -> &SPValue {
//         match self {
//             OLDStateValue::SPValue(v) => v,
//             OLDStateValue::OLDDelay(d) => &d.current_value,
//             OLDStateValue::OLDNext(n) => &n.OLDnext_value,
//             OLDStateValue::Unknown => &SPValue::Unknown,
//         }
//     }
// }

// impl OLDAssignState {
//     /// Consumes and merges the OLDAssignState into this. Return true if a key was overwritten
//     pub fn merge(&mut self, x: OLDAssignState) {
//         self.s.extend(x.s);
//     }

//     /// Checks if the two OLDAssignStates overlap. Not very good if they do. Check before merge
//     pub fn will_overwrite(&self, x: &OLDAssignState) -> bool {
//         self.s.keys().all(|k| x.s.contains_key(k))
//     }
// }



// impl OLDToStateValue for bool {
//     fn to_oldstate(&self) -> OLDStateValue {
//         OLDStateValue::SPValue(self.to_spvalue())
//     }
// }
// impl OLDToStateValue for i32 {
//     fn to_oldstate(&self) -> OLDStateValue {
//         OLDStateValue::SPValue(self.to_spvalue())
//     }
// }
// impl OLDToStateValue for String {
//     fn to_oldstate(&self) -> OLDStateValue {
//         OLDStateValue::SPValue(self.to_spvalue())
//     }
// }
// impl OLDToStateValue for &str {
//     fn to_oldstate(&self) -> OLDStateValue {
//         OLDStateValue::SPValue(self.to_spvalue())
//     }
// }
// impl<T> OLDToStateValue for Vec<T>
// where
//     T: ToSPValue,
// {
//     fn to_oldstate(&self) -> OLDStateValue {
//         OLDStateValue::SPValue(self.to_spvalue())
//     }
// }

