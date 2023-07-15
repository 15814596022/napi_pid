#![deny(clippy::all)]

use std::collections::HashMap;

use sysinfo::{PidExt, ProcessExt, System, SystemExt, Process};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(object)]
pub struct SingleProcess {
  pub pid: u32,
  pub name: String,
}

#[napi]
pub fn get_all() -> Vec<SingleProcess> {
  let sys = System::new_all();
  let mut list: Vec<SingleProcess> = vec![];
  for (pid, process) in sys.processes() {
    list.push(SingleProcess {
      pid: pid.as_u32(),
      name: process.name().to_owned(),
    })
  }
  list
}

#[napi]
pub fn get_parent(pid: u32) -> Option<SingleProcess> {
  let sys = System::new_all();
  let mut hashmap: HashMap<u32, &Process> = HashMap::new();
  for (pid, process) in sys.processes() {
    let current_pid = pid.as_u32();
    hashmap.insert(
      current_pid,
      process,
    );
  }

  if let Some(current_process) = hashmap.get(&pid) {
    if let Some(parent_pid) = current_process.parent() {
      let parent_pid_u32 = parent_pid.as_u32();
      if let Some(parent_process) = hashmap.get(&parent_pid_u32) {
        return Some(SingleProcess {
          pid: parent_pid_u32,
          name: parent_process.name().to_owned(),
        });
      }
    }
  }
  None
}

#[napi]
pub fn get_all_in_pid_list(pid: u32) -> Vec<SingleProcess> {
  let mut list: Vec<SingleProcess> = vec![];

  let sys = System::new_all();
  let mut hashmap: HashMap<u32, Vec<&Process>> = HashMap::new();
  for (_, process) in sys.processes() {
    if let Some(parent_pid) = process.parent() {
      let parent_pid_u32 = parent_pid.as_u32();
      if hashmap.contains_key(&parent_pid_u32) {
        let mut p_list: Vec<&Process> = hashmap.get(&parent_pid_u32).unwrap().to_vec();
        p_list.push(process);
        hashmap.insert(parent_pid_u32, p_list);
      } else {
        hashmap.insert(
          parent_pid_u32,
          vec![process]
        );
      }
    }
  }

  get_child_with_pid(&mut list, &hashmap, pid);
  
  list
}

fn get_child_with_pid(list: &mut Vec<SingleProcess>, hashmap: &HashMap<u32, Vec<&Process>>, pid: u32) {
  if hashmap.contains_key(&pid) {
    let children = hashmap.get(&pid).unwrap();
    for item in children {
      let child_pid = item.pid().as_u32();
      list.push(SingleProcess { pid: child_pid, name: item.name().to_owned() });
      get_child_with_pid(list, hashmap, child_pid);
    }
  }
}

#[napi(object)]
pub struct TreeProcess {
  pub pid: u32,
  pub name: String,
  pub parent_pid: u32,
  pub children: Vec<TreeProcess>,
}
