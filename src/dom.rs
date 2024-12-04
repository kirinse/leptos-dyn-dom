#[cfg(any(feature="csr",feature="hydrate"))]
use leptos::{web_sys::{Node,Element},prelude::{Mountable, Owner, Render}, IntoView};
#[cfg(any(feature="csr",feature="hydrate"))]
use leptos::wasm_bindgen::JsCast;

/// Iterates over the node and its children (DFS) and replaces elements via the given function.
#[cfg(any(feature="csr",feature="hydrate"))]
pub fn hydrate_node<
  V:IntoView+'static,
  R:FnOnce() -> V,
  F:Fn(&Element) -> Option<R>+'static
>(node:Node,replace:&F) {
  // Check node returns a new index if it replaced the node, otherwise None.
  if check_node(&node,&node,replace).0 {return}
  crate::cleanup(node.clone());
  hydrate_children(node, replace);
}

/*
/// Iterates over the children of a node and replaces elements via the given function.
#[cfg(any(feature="csr",feature="hydrate"))]
#[inline]
pub fn hydrate_children<V:IntoView+'static>(node:Node,replace:&impl Fn(&Element) -> Option<V>) {
  hydrate_children_i(node,replace);
}
   */


/// Iterates over the children of a node and replaces elements via the given function.
#[cfg(any(feature="csr",feature="hydrate"))]
pub(crate) fn hydrate_children<
  V:IntoView+'static,
  R:FnOnce() -> V,
  F:Fn(&Element) -> Option<R>+'static
>(node:Node,replace:&F) {
  let Some(mut current) = node.first_child() else { return };
  while let (_,Some(next)) = check_node(&current, &node, replace) {
    current = next;
  }
}
#[cfg(any(feature="csr",feature="hydrate"))]
fn next(top:&Node,current: &Node) -> Option<Node> {
  if let Some(c) = current.first_child() {
    return Some(c)
  }
  next_non_child(top,current)
}
#[cfg(any(feature="csr",feature="hydrate"))]
fn next_non_child(top:&Node,current: &Node) -> Option<Node> {
  if let Some(c) = current.next_sibling() {
    return Some(c)
  }
  let mut current = current.clone();
  loop {
    if let Some(p) = current.parent_node() {
      if p == *top {return None }
      if let Some(c) = p.next_sibling() {
        return Some(c)
      }
      current = p;
      continue
    }
    return None
  }
}

// Actually replaces nodes:
#[cfg(any(feature="csr",feature="hydrate"))]
fn check_node<
  V:IntoView+'static,
  R:FnOnce() -> V,
  F:Fn(&Element) -> Option<R>+'static
>(node:&Node,top:&Node,replace:&F) -> (bool,Option<Node>) {
  //leptos::logging::log!("Checking: {}",crate::prettyprint(node));
  if let Some(e) = node.dyn_ref::<Element>() {
    if let Some(v) = replace(e) {
      let p = e.parent_element().unwrap();
      let next = e.next_sibling();
      let ret = next_non_child(top, node);
      //leptos::logging::log!("Triggered! Parent: {:?}",p.outer_html());
      e.remove();
      //leptos::logging::log!("Next: {:?}",next.as_ref().map(crate::prettyprint));
      let owner = Owner::new();
      owner.with(|| {
          let mut r = v().into_view().build();
          if let Some(e) = next.as_ref() {
            e.insert_before_this(&mut r);
          } else {
            r.mount(&p,None);
          }
          let mut r = send_wrapper::SendWrapper::new(r);
          Owner::on_cleanup(|| {r.unmount();drop(r)});
      });
      Owner::on_cleanup(move || drop(owner));
      return (true,ret);
    }
  }
  (false,next(top,node))
}