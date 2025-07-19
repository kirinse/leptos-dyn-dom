#![allow(unused_variables)]
#![allow(unused_mut)]

use leptos::attr::any_attribute::AnyAttribute;
use leptos::{prelude::*, web_sys::Element};

/// Represents the original children some node in the DOM had, to be used in the [`DomChildren`](super::DomChildren), [`DomChildrenCont`](super::DomChildrenCont) and [`DomStringCont`](super::DomStringCont) components.
#[derive(Clone)]
pub struct OriginalNode {
    pub(crate) inner: send_wrapper::SendWrapper<Element>,
    attrs: Vec<AnyAttribute>,
    orig_style: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    orig_classes: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl std::ops::Deref for OriginalNode {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub(crate) struct PatchedNode(web_sys::Node);

pub(crate) struct PlainNode(send_wrapper::SendWrapper<PatchedNode>);

impl<E: Into<Element>> From<E> for OriginalNode {
    #[inline]
    fn from(value: E) -> Self {
        Self::new(value.into())
    }
}

impl OriginalNode {
    fn new(_e: Element) -> Self {
        OriginalNode {
            inner: send_wrapper::SendWrapper::new(_e.clone()),
            attrs: Vec::new(),
            orig_style: std::sync::Arc::new(std::sync::Mutex::new(None)),
            orig_classes: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub(crate) fn child_vec(&self) -> Vec<leptos::either::Either<Self, PlainNode>> {
        use leptos::wasm_bindgen::JsCast;
        let mut i = 0;
        let mut ret = Vec::new();
        while let Some(c) = self.child_nodes().get(i) {
            i += 1;
            ret.push(match c.dyn_into::<Element>() {
                Ok(e) => leptos::either::Either::Left(Self {
                    inner: send_wrapper::SendWrapper::new(e),
                    attrs: Vec::new(),
                    orig_style: std::sync::Arc::new(std::sync::Mutex::new(None)),
                    orig_classes: std::sync::Arc::new(std::sync::Mutex::new(None)),
                }),
                Err(n) => leptos::either::Either::Right(PlainNode(send_wrapper::SendWrapper::new(
                    PatchedNode(n),
                ))),
            });
        }
        ret
    }

    #[inline]
    #[allow(unused_mut)]
    pub(crate) fn as_view<F: FnMut(&mut Element) + 'static + Send>(
        &self,
        mut cont: F,
    ) -> impl IntoView + use<F> {
        let mut slf = self.clone();
        cont(&mut slf.inner);
        slf
    }

    pub fn deep_clone(&self) -> Self {
        use leptos::wasm_bindgen::JsCast;
        Self::new(
            self.inner
                .clone_node_with_deep(true)
                .expect("Failed to clone node")
                .dyn_into()
                .unwrap_or_else(|_| unreachable!()),
        )
    }

    #[inline]
    pub fn inner_html(&self) -> String {
        self.inner.inner_html()
    }
    #[inline]
    pub fn html_string(&self) -> String {
        self.inner.outer_html()
    }

    fn style_attr<NewAttr: leptos::attr::Attribute>(
        attr: NewAttr,
        orig_style: &mut Option<String>,
        e: &leptos::web_sys::Element,
    ) -> leptos::tachys::html::attribute::any_attribute::AnyAttribute {
        use leptos::tachys::html::attribute::any_attribute::IntoAnyAttribute;
        if orig_style.is_none() {
            if let Some(o) = e.get_attribute("style") {
                *orig_style = Some(o);
            } else {
                *orig_style = Some(String::new());
            };
        }
        let orig_style = orig_style.as_ref().unwrap_or_else(|| unreachable!()).trim();
        if orig_style.is_empty() {
            return attr.into_any_attr();
        }

        let mut buf = String::new();
        let mut class = String::new();
        let mut style = String::new();
        let mut inner_html = String::new();
        attr.to_html(&mut buf, &mut class, &mut style, &mut inner_html);
        if !style.ends_with(';') {
            style.push(';');
        }
        style.push_str(orig_style);
        leptos::tachys::html::style::style(style).into_any_attr() //.build(e);
    }

    fn class_attr<NewAttr: leptos::attr::Attribute>(
        attr: NewAttr,
        orig_classes: &mut Option<String>,
        e: &leptos::web_sys::Element,
    ) -> leptos::tachys::html::attribute::any_attribute::AnyAttribute {
        use leptos::tachys::html::attribute::any_attribute::IntoAnyAttribute;
        //let mut orig_classes = self.orig_classes.lock().expect("Failed to lock classes");
        if orig_classes.is_none() {
            if let Some(o) = e.get_attribute("class") {
                *orig_classes = Some(o);
            } else {
                *orig_classes = Some(String::new());
            };
        }
        let orig_classes = orig_classes
            .as_ref()
            .unwrap_or_else(|| unreachable!())
            .trim();
        if orig_classes.is_empty() {
            return attr.into_any_attr();
        }

        let mut buf = String::new();
        let mut class = String::new();
        let mut style = String::new();
        let mut inner_html = String::new();
        attr.to_html(&mut buf, &mut class, &mut style, &mut inner_html);
        if !class.ends_with(' ') {
            class.push(' ');
        }
        class.push_str(orig_classes);
        leptos::tachys::html::class::class(class).into_any_attr() //.build(e);
    }
}

mod leptos_impl {
    use super::{OriginalNode, PlainNode};
    use leptos::attr::Attribute;
    use leptos::{
        attr::{
            any_attribute::{AnyAttribute, AnyAttributeState},
            //Attribute,
        },
        prelude::*,
    };
    use web_sys::Element;

    impl Mountable for super::PatchedNode {
        fn mount(
            &mut self,
            parent: &leptos::tachys::renderer::types::Element,
            marker: Option<&leptos::tachys::renderer::types::Node>,
        ) {
            self.0.mount(parent, marker)
        }
        fn unmount(&mut self) {
            if let Some(n) = self.0.parent_node() {
                let _ = n.remove_child(&n);
            }
        }
        fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
            self.0.insert_before_this(child)
        }
        fn elements(&self) -> Vec<leptos::tachys::renderer::types::Element> {
            self.0.elements()
        }
    }

    impl Render for PlainNode {
        type State = super::PatchedNode;
        #[inline]
        fn build(self) -> Self::State {
            self.0.take()
        }
        #[inline]
        fn rebuild(self, _state: &mut Self::State) {}
    }

    impl AddAnyAttr for PlainNode {
        type Output<SomeNewAttr: leptos::attr::Attribute> = Self;
        fn add_any_attr<NewAttr: leptos::attr::Attribute>(
            self,
            _: NewAttr,
        ) -> Self::Output<NewAttr> {
            self
        }
    }

    pub struct MountableNode {
        inner: Element,
        attrs: Vec<AnyAttributeState>,
    }
    impl Mountable for MountableNode {
        fn mount(
            &mut self,
            parent: &leptos::tachys::renderer::types::Element,
            marker: Option<&leptos::tachys::renderer::types::Node>,
        ) {
            self.inner.mount(parent, marker)
        }
        fn unmount(&mut self) {
            self.inner.unmount()
        }
        fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
            self.inner.insert_before_this(child)
        }
        fn elements(&self) -> Vec<leptos::tachys::renderer::types::Element> {
            vec![self.inner.clone()]
        }
    }

    impl Render for OriginalNode {
        type State = MountableNode;
        #[inline]
        fn build(self) -> Self::State {
            let inner = self.inner.take();
            let attrs = self
                .attrs
                .into_iter()
                .map(|a| {
                    if is_style(&a) {
                        Self::style_attr(
                            a,
                            &mut self.orig_style.lock().expect("failed to lock style"),
                            &inner,
                        )
                        .build(&inner)
                    } else if is_class(&a) {
                        Self::class_attr(
                            a,
                            &mut self.orig_classes.lock().expect("failed to lock class"),
                            &inner,
                        )
                        .build(&inner)
                    } else {
                        a.build(&inner)
                    }
                })
                .collect();
            MountableNode { inner, attrs }
        }
        #[inline]
        fn rebuild(self, state: &mut Self::State) {
            for (a, s) in self.attrs.into_iter().zip(state.attrs.iter_mut()) {
                if is_style(&a) {
                    Self::style_attr(
                        a,
                        &mut self.orig_style.lock().expect("failed to lock style"),
                        &self.inner,
                    )
                    .rebuild(s);
                } else if is_class(&a) {
                    Self::class_attr(
                        a,
                        &mut self.orig_classes.lock().expect("failed to lock class"),
                        &self.inner,
                    )
                    .rebuild(s);
                } else {
                    a.rebuild(s);
                }
            }
        }
    }

    impl AddAnyAttr for OriginalNode {
        type Output<SomeNewAttr: leptos::attr::Attribute> = Self;
        fn add_any_attr<NewAttr: leptos::attr::Attribute>(
            mut self,
            attr: NewAttr,
        ) -> Self::Output<NewAttr> {
            use leptos::tachys::html::attribute::any_attribute::IntoAnyAttribute;
            self.attrs.push(attr.into_any_attr());
            self
        }
    }

    fn is_style<Attr: leptos::attr::Attribute>(attr: &Attr) -> bool {
        let name = std::any::type_name_of_val(&attr);
        name.starts_with("tachys::html::style::Style")
    }

    fn is_class<Attr: leptos::attr::Attribute>(attr: &Attr) -> bool {
        let name = std::any::type_name_of_val(&attr);
        name.starts_with("tachys::html::class::Class")
    }

    impl RenderHtml for OriginalNode {
        type AsyncOutput = Self;
        type Owned = Self;
        const MIN_LENGTH: usize = 0;
        fn into_owned(self) -> Self::Owned {
            self
        }
        fn dry_resolve(&mut self) {
            for a in self.attrs.iter_mut() {
                a.dry_resolve();
            }
        }
        fn resolve(self) -> impl std::future::Future<Output = Self::AsyncOutput> + Send {
            std::future::ready(self)
        }
        fn to_html_with_buf(
            self,
            _: &mut String,
            _: &mut leptos::tachys::view::Position,
            _: bool,
            _: bool,
            _: Vec<AnyAttribute>,
        ) {
        }

        fn hydrate<const FROM_SERVER: bool>(
            self,
            _cursor: &leptos::tachys::hydration::Cursor,
            _position: &leptos::tachys::view::PositionState,
        ) -> Self::State {
            self.inner.take();
            todo!()
        }
    }

    // basically unreachable, because only relevant in SSR
    impl RenderHtml for PlainNode {
        type AsyncOutput = Self;
        type Owned = Self;

        fn into_owned(self) -> Self::Owned {
            self
        }

        const MIN_LENGTH: usize = 0;
        fn dry_resolve(&mut self) {}
        fn resolve(self) -> impl std::future::Future<Output = Self::AsyncOutput> + Send {
            std::future::ready(self)
        }
        fn to_html_with_buf(
            self,
            _: &mut String,
            _: &mut leptos::tachys::view::Position,
            _: bool,
            _: bool,
            _: Vec<AnyAttribute>,
        ) {
        }

        fn hydrate<const FROM_SERVER: bool>(
            self,
            _cursor: &leptos::tachys::hydration::Cursor,
            _position: &leptos::tachys::view::PositionState,
        ) -> Self::State {
            self.0.take()
        }
    }
}
