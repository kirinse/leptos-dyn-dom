use leptos::prelude::*;
use leptos::web_sys::Element;
use wasm_bindgen::prelude::*;

use leptos_dyn_dom::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    hydrate_body(|orig| view!(<MainBody orig/>).into_any())
}

#[component]
fn MainBody(orig: OriginalNode) -> impl IntoView {
    use thaw::ConfigProvider;
    //leptos::logging::log!("Here (body)");
    view! {
        <ConfigProvider>
            <DomChildrenCont orig cont=replace/>
        </ConfigProvider>
    }
}

#[component]
fn MyReplacementComponent<Ch: IntoView + 'static>(children: TypedChildrenMut<Ch>) -> impl IntoView {
    use thaw::*;
    let mut children = children.into_inner();
    let show_class = RwSignal::new("foo-bar");
    let on_click = move |_| {
        show_class.update(|s| {
            if *s == "foo-bar" {
                *s = "moo"
            } else {
                *s = "foo-bar"
            }
        })
    };
    view! {
        <button on:click=on_click>{"Switch classes"}</button>
        //<div><div style="border: 1px solid red;width:fit-content;margin:auto">
          <Popover>
              <PopoverTrigger slot>
                  <span style="display:contents">{move || {
                    leptos::logging::log!("Rendering children");
                    children()
                        .add_any_attr(leptos::tachys::html::style::style("border: 1px solid red"))
                        .add_any_attr(leptos::tachys::html::class::class(show_class))
                        .add_any_attr(leptos::tachys::html::attribute::custom::custom_attribute("data-foo","bar"))
                  }}</span>
                  //<DomChildrenCont orig cont=replace/>
              </PopoverTrigger>
              <div style="border: 1px solid black;font-weight:bold;">"IT WORKS!"</div>
          </Popover>
       //</div></div>
    }
}

fn replace(e: &Element) -> Option<impl FnOnce() -> AnyView> {
    e.get_attribute("data-replace-with-leptos").map(|_| {
        let orig: OriginalNode = e.clone().into();
        || {
            leptos::web_sys::console::log_2(
                &leptos::wasm_bindgen::JsValue::from_str("Hydrating node"),
                &orig,
            );
            view!(<MyReplacementComponent>
            <DomCont orig=orig.clone() cont=replace skip_head=true/>
        </MyReplacementComponent>)
            .into_any()
        }
    })
}
