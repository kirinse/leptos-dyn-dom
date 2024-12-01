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
    leptos::logging::log!("Here (body)");
    view! {
        <ConfigProvider>
            {orig.children_into_view_cont(replace,None)}
            //<DomChildrenCont orig cont=replace />
        </ConfigProvider>
    }
}

#[component]
fn MyReplacementComponent(children: Children) -> impl IntoView {
    use thaw::*;
    view! {
        <div><div style="border: 1px solid red;width:fit-content;margin:auto">
          <Popover>
              <PopoverTrigger slot>
                  {children()}
                  //<DomChildrenCont orig cont=replace/>
              </PopoverTrigger>
              <div style="border: 1px solid black;font-weight:bold;">"IT WORKS!"</div>
          </Popover>
       </div></div>
    }
}

fn replace(e: &Element) -> Option<impl IntoView> {
    e.get_attribute("data-replace-with-leptos").map(|_| {
        let orig: OriginalNode = e.clone().into();
        view!(<MyReplacementComponent>
            {orig.children_into_view_cont(replace,None)}
            </MyReplacementComponent>)
    })
}
