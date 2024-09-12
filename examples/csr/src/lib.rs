use leptos::prelude::*;
use tachys::view::any_view::AnyView;
use wasm_bindgen::prelude::*;

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
            <DomChildrenCont orig f=replace />
        </ConfigProvider>
    }
}

#[component]
fn MyReplacementComponent(orig:OriginalChildren) -> impl IntoView {
    use thaw::*;
   view! {
      <div><div style="border: 1px solid red;width:fit-content;margin:auto">
        <Popover>
            <PopoverTrigger slot>
                <DomChildrenCont orig f=replace/>
            </PopoverTrigger>
            <div style="border: 1px solid black;font-weight:bold;">"IT WORKS!"</div>
        </Popover>
     </div></div>
  }
}

fn replace(e:&Element) -> Option<AnyView<Dom>> {
    e.get_attribute("data-replace-with-leptos").map(|_| {
        let orig = OriginalChildren::new(e);
        view!(<MyReplacementComponent orig/>).into_any()
    })
}