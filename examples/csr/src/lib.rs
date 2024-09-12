use leptos::prelude::*;
use tachys::view::any_view::AnyView;
use wasm_bindgen::prelude::*;
//use web_sys::{Element, HtmlElement};

mod utils;
use utils::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use leptos_dyn_dom::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    hydrate_body(|orig| view!(<MainBody orig/>).into_any())
}

#[component]
fn MainBody(orig: OriginalChildren) -> impl IntoView {
    use thaw::ConfigProvider;
    leptos::logging::log!("Here (body)");
    view! {
        <ConfigProvider>
            //<DomChildren elem=body/>
            <DomChildrenCont orig f=replace />
        </ConfigProvider>
    }
}

fn replace(e: &Element) -> Option<AnyView<Dom>> {
    e.get_attribute("data-replace-with-leptos").map(|_| {
        let orig = OriginalChildren::new(e);
        let tag_name = e.tag_name();
        leptos::logging::log!("Replacing {}", tag_name);
        if let Some(t) = is_mathml(&tag_name) {
            leptos::logging::log!("Math: {}", tag_name);
            view!(<ReplacementComponent orig in_math=t/>).into_any()
        } else {
            leptos::logging::log!("Non-math: {}", tag_name);
            view!(<ReplacementComponent orig/>).into_any()
        }
    })
}

#[component]
fn ReplacementComponent(
    orig: OriginalChildren,
    #[prop(optional)] in_math: Option<&'static str>,
) -> impl IntoView {
    use thaw::*;
    if let Some(tag) = in_math {
        leptos::logging::log!("Here (math)");
        view! {
            <mrow>
                <mtext><Popover><PopoverTrigger slot>"This is a Leptos component"</PopoverTrigger><div>"Test in Math!"</div></Popover></mtext>
                <mrow class="my-class">
                    <MathMLTag tag><DomChildrenCont orig f=replace/></MathMLTag>
                </mrow>
            </mrow>
        }
        .into_any()
    } else {
        leptos::logging::log!("Here (html)");
        view! {<Popover>
            <PopoverTrigger slot>
                "This is a Leptos component"
                <div class="my-class"><DomChildrenCont orig f=replace/></div>
            </PopoverTrigger>
            <div>"TEST!"</div>
        </Popover>}
        .into_any()
    }
}

/*

static DID_BODY : std::sync::OnceLock<()> = std::sync::OnceLock::new();

use leptos_dyn_hydrate::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();

    hydrate_dom(|n| {
        let in_math = is_mathml(&n.tag_name());
        if DID_BODY.get().is_none() && n.tag_name().eq_ignore_ascii_case("body") {
            leptos::logging::log!("Doing body");
            DID_BODY.get_or_init(|| ());
            Some(view!(<MainBody/>).into_any())
        } else {
            n.get_attribute("data-replace-with-leptos").map(|_| {
                leptos::logging::log!("Replacing {}",n.tag_name());
                view!(<ReplacementComponent in_math/>).into_any()
            })
        }
    });
}

#[component]
fn MainBody() -> impl IntoView {
    use thaw::ConfigProvider;
    view!{
        <body><ConfigProvider>
            <DomChildren/>
        </ConfigProvider></body>
    }
}


#[component]
fn ReplacementComponent(#[prop(optional)] in_math:bool) -> impl IntoView {
    use thaw::*;
    if in_math {
        leptos::logging::log!("Here (math)");
        view! {
            <mrow>
                <mtext>"This is a Leptos component"</mtext>
                <mrow class="my-class"><DomChildren in_math/></mrow>
            </mrow>
        }
        .into_any()
    } else {
        leptos::logging::log!("Here (html)");
        view! {<Popover>
            <PopoverTrigger slot>
                "This is a Leptos component"
                <span class="my-class"><DomChildren/></span>
            </PopoverTrigger>
            <div>"TEST!"</div>
        </Popover>}
        .into_any()
    }
}

*/

/*
static CONTEXT: std::sync::OnceLock<Owner> = std::sync::OnceLock::<Owner>::new();
fn with_context<R>(f: impl FnOnce() -> R) -> R {
    CONTEXT.get_or_init(Owner::new).with(f)
}

fn mount() {
    with_context(|| {
        let body = (**tachys::dom::body()).clone();
        body.iter_dom(|_| Ok::<_, ()>(()), |_| Ok::<_, ()>(()))
    });
}

fn pass_in_children(parent: Element, previous: Element) {
    leptos::logging::log!("Here: {previous:?}");
    let children = previous.child_nodes();
    leptos::logging::log!("Children: {children:?}");

    while let Some(c) = children.item(0) {
        leptos::logging::log!("Appending: {c:?}");
        parent.append_child(&c).unwrap();
    }
}

#[component]
fn ReplacementComponent(element: Element) -> impl IntoView {
    use thaw::*;
    let tag = element.tag_name();
    if is_mathml(&tag) {
        view! {
            <mrow>
                <mtext>"This is a Leptos component"</mtext>
                <mrow use:pass_in_children=element/>
            </mrow>
        }
        .into_any()
    } else {
        leptos::logging::log!("Here 1");
        let s = view!(<span use:pass_in_children=element/>);
        view! {<Popover>
            <PopoverTrigger slot>
                "This is a Leptos component"
                {s}
            </PopoverTrigger>
            <div>"TEST!"</div>
        </Popover>}
        .into_any()
    }
}

fn replace_nodes() {
    replace_nodes_with_attribute("data-replace-with-leptos");
}

fn replace_nodes_with_attribute(attribute: &str) {
    leptos::logging::log!("Here!");
    let elements = tachys::dom::document()
        .query_selector_all(&format!("[{}]", attribute))
        .expect("error querying elements");
    let cb = Closure::<dyn Fn(_)>::new(|element: web_sys::Node| {
        if let Ok(element) = element.dyn_into::<Element>() {
            replace_with_leptos_component(element);
        }
    });
    elements.for_each(cb.as_ref().unchecked_ref()).unwrap();
    /*
    for i in 0..elements.length() {
        if let Some(element) = elements.item(i) {
            if let Ok(element) = element.dyn_into::<Element>() {
                replace_with_leptos_component(element);
            }
        }
    }*/
}

fn replace_with_leptos_component(element: Element) {
    if let Some(parent) = element.parent_node() {
        let div = document()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .expect("Failed to convert to HtmlElement");
        leptos::logging::log!("Mounting to {div:?}");
        let elem = element.clone();
        mount_to(
            div.clone(),
            move || view! { <ReplacementComponent element/> },
        )
        .forget();
        let e = div.last_child().unwrap();
        leptos::logging::log!("Replacing {elem:?} by {e:?}");
        parent.replace_child(&e, &elem).unwrap();

    /*
    if let Ok(html_parent) = parent.clone().dyn_into::<HtmlElement>() {
        let e = element.clone();
        leptos::logging::log!("Mounting to {e:?}");
        mount_to(
            html_parent,
            move || view! { <ReplacementComponent element/> }
        ).forget();
        leptos::logging::log!("Moving component");
        let elem = parent.last_child().unwrap();
        parent.replace_child(&elem, &e).unwrap();
    } else {
        leptos::logging::log!("parent is no html element: {parent:?}");
    }
    */
    } else {
        leptos::logging::log!("element has no parent: {element:?}");
    }
}
*/
