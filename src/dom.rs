use web_sys::Element;

#[cfg(any(feature="csr",feature="hydrate"))]
use web_sys::Node;
#[cfg(any(feature="csr",feature="hydrate"))]
use leptos::{prelude::{Dom, Mountable, Owner, Render}, tachys::view::any_view::AnyView, IntoView};

#[cfg(any(feature="csr",feature="hydrate"))]
use wasm_bindgen::JsCast;

#[cfg(any(feature="csr",feature="hydrate"))]
macro_rules! log {
    ($($arg:tt)*) => { if $crate::LOG { ::leptos::logging::log!($($arg)*) } }
}

pub struct OriginalChildren(
  #[cfg(any(feature="csr",feature="hydrate"))]
  pub(crate) send_wrapper::SendWrapper<Vec<Node>>
);
impl OriginalChildren {
  pub fn new(_e:&Element) -> Self {
    #[cfg(any(feature="csr",feature="hydrate"))]
    {
      let mut vec = Vec::new();
      while let Some(c) = _e.child_nodes().item(0) {
        let _ = _e.remove_child(&c);
        vec.push(c);
      }
      OriginalChildren(send_wrapper::SendWrapper::new(vec))
    }
    #[cfg(not(any(feature="csr",feature="hydrate")))]
    { OriginalChildren() }
  }
}

#[cfg(any(feature="csr",feature="hydrate"))]
pub fn hydrate_node(node:Node,replace:&impl Fn(&Element) -> Option<AnyView<Dom>>) {
  if check_node(node.clone(),0,replace).is_some() {return}
  let mut current = node;
  let mut index = 0u32;
  let mut stack : Vec<(Node,u32)> = Vec::new();
  log!("Iterating over {current:?}");
  loop {
    if let Some(c) = current.child_nodes().item(index) {
      log!("Iterator: checking {c:?}");
      if let Some(skip) = check_node(c.clone(),index,replace) {
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

#[cfg(any(feature="csr",feature="hydrate"))]
fn check_node(node:Node,mut start:u32,replace:&impl Fn(&Element) -> Option<AnyView<Dom>>) -> Option<u32> {
  if let Ok(e) = node.dyn_into::<Element>() {
    if let Some(v) = replace(&e) {
      log!("Building view");
      let mut r = v.into_view().build();
      log!("Inserting new element");
      e.insert_before_this(&mut r);
      let r = send_wrapper::SendWrapper::new(r);
      Owner::on_cleanup(move|| {drop(r)});
      log!("Getting parent");
      let p = e.parent_node().unwrap();
      while let Some(c) = p.child_nodes().item(start) {
        if c == *e {
          log!("New index found");
          break
        }
        start += 1;
      }
      log!("removing old element");
      e.remove();
      return Some(start);
    }
  }
  None
}