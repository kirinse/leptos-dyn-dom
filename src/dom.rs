use leptos::tachys::reactive_graph::OwnedView;
use leptos::wasm_bindgen::JsCast;
use leptos::{
    IntoView,
    prelude::{Mountable, Owner, Render},
    web_sys::{Element, Node},
};

/// Iterates over the node and its children (DFS) and replaces elements via the given function.
pub fn hydrate_node<
    V: IntoView + 'static,
    R: FnOnce() -> V,
    G: FnOnce() + 'static,
    F: Fn(&Element) -> (Option<R>, Option<G>) + 'static + Send,
>(
    node: Node,
    replace: &F,
) {
    let mut thens = Vec::with_capacity(1);
    let (r, _) = check_node(&node, &node, replace, &mut thens);
    // Check node returns a new index if it replaced the node, otherwise None.
    if r {
        return;
    }
    //crate::cleanup(node.clone());
    hydrate_children(node, replace);
    if let Some(then) = thens.pop().flatten() {
        then();
    }
}

/// Iterates over the children of a node and replaces elements via the given function.
pub(crate) fn hydrate_children<
    V: IntoView + 'static,
    R: FnOnce() -> V,
    G: FnOnce() + 'static,
    F: Fn(&Element) -> (Option<R>, Option<G>) + 'static + Send,
>(
    node: Node,
    replace: &F,
) {
    let mut continues: Vec<Option<G>> = Vec::new();
    let Some(mut current) = node.first_child() else {
        return;
    };
    while let (_, Some(next)) = check_node(&current, &node, replace, &mut continues) {
        current = next;
    }
}

fn next<G: FnOnce() + 'static>(
    top: &Node,
    current: &Node,
    then: Option<G>,
    continues: &mut Vec<Option<G>>,
) -> Option<Node> {
    if let Some(c) = current.first_child() {
        continues.push(then);
        return Some(c);
    }
    if let Some(then) = then {
        then();
    }
    let (r, thens) = next_non_child(top, current, continues);
    for then in thens {
        then()
    }
    r
}

fn next_non_child<G: FnOnce() + 'static>(
    top: &Node,
    current: &Node,
    continues: &mut Vec<Option<G>>,
) -> (Option<Node>, Vec<G>) {
    let mut thens = Vec::new();
    if let Some(c) = current.next_sibling() {
        return (Some(c), thens);
    }
    let mut current = current.clone();
    loop {
        if let Some(then) = continues.pop().flatten() {
            thens.push(then)
        }
        if let Some(p) = current.parent_node() {
            if p == *top {
                return (None, thens);
            }
            if let Some(c) = p.next_sibling() {
                return (Some(c), thens);
            }
            current = p;
            continue;
        }
        return (None, thens);
    }
}

// Actually replaces nodes:
fn check_node<
    V: IntoView + 'static,
    R: FnOnce() -> V,
    G: FnOnce() + 'static,
    F: Fn(&Element) -> (Option<R>, Option<G>) + 'static + Send,
>(
    node: &Node,
    top: &Node,
    replace: &F,
    continues: &mut Vec<Option<G>>,
) -> (bool, Option<Node>) {
    //leptos::logging::log!("Checking: {}",crate::prettyprint(node));
    if let Some(e) = node.dyn_ref::<Element>() {
        let (r, then) = replace(e);
        if let Some(v) = r {
            let parent = e.parent_element().unwrap();
            let next_sibling = e.next_sibling();
            let (ret, thens) = next_non_child(top, node, continues);
            let owner = Owner::current().expect("not in a reactive context").child();
            let v = owner.with(v);
            let mut r = OwnedView::new_with_owner(v, owner.clone()).build();
            if let Some(e) = next_sibling.as_ref() {
                e.insert_before_this(&mut r);
            } else {
                r.mount(&parent, None);
            }
            let r = send_wrapper::SendWrapper::new(r);
            Owner::on_cleanup(move || drop(r));
            if let Some(then) = then {
                owner.with(then);
            }
            for then in thens {
                then();
            }
            (true, ret)

            /*
            //leptos::logging::log!("Triggered! Parent: {:?}",p.outer_html());
            e.remove();
            let ne: SendWrapper<Element> = send_wrapper::SendWrapper::new(
                e.clone_node_with_deep(true)
                    .expect("Element disappeared")
                    .dyn_into()
                    .unwrap_or_else(|_| unreachable!()),
            );
            //leptos::logging::log!("Next: {:?}",next.as_ref().map(crate::prettyprint));
            let owner = Owner::new();
            owner.with(move || {
                let mut r = v().into_view().build();
                if let Some(e) = next.as_ref() {
                    e.insert_before_this(&mut r);
                } else {
                    r.mount(&p, None);
                }
                let mut r = send_wrapper::SendWrapper::new(r);
                let p = send_wrapper::SendWrapper::new(p);
                let next = send_wrapper::SendWrapper::new(next);
                Owner::on_cleanup(move || {
                    /*leptos::web_sys::console::log_2(
                        &leptos::wasm_bindgen::JsValue::from_str("Cleaning up former"),
                        &ne,
                    );*/
                    r.unmount();
                    drop(r);
                    if let Some(e) = next.take().as_ref() {
                        e.insert_before_this(&mut ne.take());
                    } else {
                        ne.take().mount(&p.take(), None);
                    }
                });
            });
            Owner::on_cleanup(move || drop(owner));
            if let Some(then) = then {
                then();
            }
            let ret = next_non_child(top, node, continues);
            (true, ret)
            */
        } else {
            (false, next(top, node, then, continues))
        }
    } else {
        (false, next::<G>(top, node, None, continues))
    }
}
