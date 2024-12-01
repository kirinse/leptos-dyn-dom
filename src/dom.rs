#[cfg(any(feature="csr",feature="hydrate"))]
use leptos::{web_sys::{Node,Element},prelude::{Mountable, Owner, Render}, IntoView};
#[cfg(any(feature="csr",feature="hydrate"))]
use wasm_bindgen::JsCast;

/// Iterates over the node and its children (DFS) and replaces elements via the given function.
#[cfg(any(feature="csr",feature="hydrate"))]
pub fn hydrate_node<V:IntoView+'static>(node:Node,replace:&impl Fn(&Element) -> Option<V>) {
  // Check node returns a new index if it replaced the node, otherwise None.
  if check_node(&node,0,replace).is_some() {return}
  hydrate_children(node, replace);
}


/// Iterates over the children of a node and replaces elements via the given function.
#[cfg(any(feature="csr",feature="hydrate"))]
pub fn hydrate_children<V:IntoView+'static>(node:Node,replace:&impl Fn(&Element) -> Option<V>) {
  // Non-recursive DOM iteration
  let mut current = node;
  let mut index = 0u32;
  let mut stack : Vec<(Node,u32)> = Vec::new();
  loop {
    if let Some(c) = current.child_nodes().item(index) {
      // Check node returns a new index if it replaced the node, otherwise None.
      if let Some(skip) = check_node(&c,index,replace) {
        index = skip;
        continue;
      }
      if c.has_child_nodes() {
        let old = std::mem::replace(&mut current,c);
        stack.push((old,index + 1));
        index = 0;
      } else { index += 1;}
    } else if let Some((old,idx)) = stack.pop() {
        current = old;
        index = idx;
    } else { break; }
  }
}

// Actually replaces nodes:
#[cfg(any(feature="csr",feature="hydrate"))]
fn check_node<V:IntoView+'static>(node:&Node,mut start:u32,replace:&impl Fn(&Element) -> Option<V>) -> Option<u32> {
  if let Some(e) = node.dyn_ref::<Element>() {
    if let Some(v) = replace(e) {
      // This is mostly copied from leptos::mount_to_body and related methods
      let mut r = v.into_view().build();
      e.insert_before_this(&mut r);
      // we need to keep the state alive. My buest guess is to hand it over to the owner to clean it up when it deems it necessary.
      let r = send_wrapper::SendWrapper::new(r);
      Owner::on_cleanup(move|| {drop(r)});
      // remove the old element and return the index at which to continue iteration
      let p = e.parent_node().unwrap();
      while let Some(c) = p.child_nodes().item(start) {
        if c == **e {
          break
        }
        start += 1;
      }
      e.remove();
      return Some(start);
    }
  }
  None
}