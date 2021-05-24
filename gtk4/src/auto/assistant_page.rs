// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::AssistantPageType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct AssistantPage(Object<ffi::GtkAssistantPage>);

    match fn {
        type_ => || ffi::gtk_assistant_page_get_type(),
    }
}

impl AssistantPage {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`AssistantPage`]
    /// This method returns an instance of [`AssistantPageBuilder`] which can be used to create a [`AssistantPage`].
    pub fn builder() -> AssistantPageBuilder {
        AssistantPageBuilder::default()
    }

    #[doc(alias = "gtk_assistant_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_assistant_page_get_child(self.to_glib_none().0)) }
    }

    pub fn is_complete(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"complete\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `complete` getter")
        }
    }

    pub fn set_complete(&self, complete: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"complete\0".as_ptr() as *const _,
                complete.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "page-type")]
    pub fn page_type(&self) -> AssistantPageType {
        unsafe {
            let mut value =
                glib::Value::from_type(<AssistantPageType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"page-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `page-type` getter")
        }
    }

    #[doc(alias = "page-type")]
    pub fn set_page_type(&self, page_type: AssistantPageType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"page-type\0".as_ptr() as *const _,
                page_type.to_value().to_glib_none().0,
            );
        }
    }

    pub fn title(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                title.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "complete")]
    pub fn connect_complete_notify<F: Fn(&AssistantPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_complete_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::complete\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_complete_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page-type")]
    pub fn connect_page_type_notify<F: Fn(&AssistantPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_type_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&AssistantPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&AssistantPage) + 'static>(
            this: *mut ffi::GtkAssistantPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`AssistantPage`].
pub struct AssistantPageBuilder {
    child: Option<Widget>,
    complete: Option<bool>,
    page_type: Option<AssistantPageType>,
    title: Option<String>,
}

impl AssistantPageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AssistantPageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AssistantPage`].
    pub fn build(self) -> AssistantPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref complete) = self.complete {
            properties.push(("complete", complete));
        }
        if let Some(ref page_type) = self.page_type {
            properties.push(("page-type", page_type));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        glib::Object::new::<AssistantPage>(&properties)
            .expect("Failed to create an instance of AssistantPage")
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = Some(complete);
        self
    }

    pub fn page_type(mut self, page_type: AssistantPageType) -> Self {
        self.page_type = Some(page_type);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl fmt::Display for AssistantPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AssistantPage")
    }
}