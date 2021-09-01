// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::PageSetup;
use crate::PaperSize;
use crate::PrintCapabilities;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkPrinter")]
    pub struct Printer(Object<ffi::GtkPrinter>);

    match fn {
        type_ => || ffi::gtk_printer_get_type(),
    }
}

impl Printer {
    //#[doc(alias = "gtk_printer_new")]
    //pub fn new(name: &str, backend: /*Ignored*/&mut PrintBackend, virtual_: bool) -> Printer {
    //    unsafe { TODO: call ffi:gtk_printer_new() }
    //}

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Printer`] objects.
    ///
    /// This method returns an instance of [`PrinterBuilder`] which can be used to create [`Printer`] objects.
    pub fn builder() -> PrinterBuilder {
        PrinterBuilder::default()
    }

    #[doc(alias = "gtk_printer_accepts_pdf")]
    pub fn accepts_pdf(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_accepts_pdf(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_accepts_ps")]
    pub fn accepts_ps(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_accepts_ps(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_compare")]
    pub fn compare(&self, b: &Printer) -> i32 {
        unsafe { ffi::gtk_printer_compare(self.to_glib_none().0, b.to_glib_none().0) }
    }

    //#[doc(alias = "gtk_printer_get_backend")]
    //#[doc(alias = "get_backend")]
    //pub fn backend(&self) -> /*Ignored*/PrintBackend {
    //    unsafe { TODO: call ffi:gtk_printer_get_backend() }
    //}

    #[doc(alias = "gtk_printer_get_capabilities")]
    #[doc(alias = "get_capabilities")]
    pub fn capabilities(&self) -> PrintCapabilities {
        unsafe { from_glib(ffi::gtk_printer_get_capabilities(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_get_default_page_size")]
    #[doc(alias = "get_default_page_size")]
    pub fn default_page_size(&self) -> PageSetup {
        unsafe {
            from_glib_full(ffi::gtk_printer_get_default_page_size(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_printer_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_printer_get_description(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_get_hard_margins")]
    #[doc(alias = "get_hard_margins")]
    pub fn hard_margins(&self) -> Option<(f64, f64, f64, f64)> {
        unsafe {
            let mut top = mem::MaybeUninit::uninit();
            let mut bottom = mem::MaybeUninit::uninit();
            let mut left = mem::MaybeUninit::uninit();
            let mut right = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_printer_get_hard_margins(
                self.to_glib_none().0,
                top.as_mut_ptr(),
                bottom.as_mut_ptr(),
                left.as_mut_ptr(),
                right.as_mut_ptr(),
            ));
            let top = top.assume_init();
            let bottom = bottom.assume_init();
            let left = left.assume_init();
            let right = right.assume_init();
            if ret {
                Some((top, bottom, left, right))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_printer_get_hard_margins_for_paper_size")]
    #[doc(alias = "get_hard_margins_for_paper_size")]
    pub fn hard_margins_for_paper_size(
        &self,
        paper_size: &mut PaperSize,
    ) -> Option<(f64, f64, f64, f64)> {
        unsafe {
            let mut top = mem::MaybeUninit::uninit();
            let mut bottom = mem::MaybeUninit::uninit();
            let mut left = mem::MaybeUninit::uninit();
            let mut right = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_printer_get_hard_margins_for_paper_size(
                self.to_glib_none().0,
                paper_size.to_glib_none_mut().0,
                top.as_mut_ptr(),
                bottom.as_mut_ptr(),
                left.as_mut_ptr(),
                right.as_mut_ptr(),
            ));
            let top = top.assume_init();
            let bottom = bottom.assume_init();
            let left = left.assume_init();
            let right = right.assume_init();
            if ret {
                Some((top, bottom, left, right))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_printer_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_printer_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_get_job_count")]
    #[doc(alias = "get_job_count")]
    pub fn job_count(&self) -> i32 {
        unsafe { ffi::gtk_printer_get_job_count(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_printer_get_location")]
    #[doc(alias = "get_location")]
    pub fn location(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_printer_get_location(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_printer_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_get_state_message")]
    #[doc(alias = "get_state_message")]
    pub fn state_message(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_printer_get_state_message(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_has_details")]
    pub fn has_details(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_has_details(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_is_accepting_jobs")]
    pub fn is_accepting_jobs(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_is_accepting_jobs(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_is_active")]
    pub fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_is_active(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_is_default")]
    pub fn is_default(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_is_default(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_is_paused")]
    pub fn is_paused(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_is_paused(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_is_virtual")]
    pub fn is_virtual(&self) -> bool {
        unsafe { from_glib(ffi::gtk_printer_is_virtual(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_printer_list_papers")]
    pub fn list_papers(&self) -> Vec<PageSetup> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_printer_list_papers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_printer_request_details")]
    pub fn request_details(&self) {
        unsafe {
            ffi::gtk_printer_request_details(self.to_glib_none().0);
        }
    }

    #[doc(alias = "details-acquired")]
    pub fn connect_details_acquired<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn details_acquired_trampoline<F: Fn(&Printer, bool) + 'static>(
            this: *mut ffi::GtkPrinter,
            success: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(success))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"details-acquired\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    details_acquired_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accepting-jobs")]
    pub fn connect_accepting_jobs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accepting_jobs_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::accepting-jobs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accepting_jobs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "job-count")]
    pub fn connect_job_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_job_count_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::job-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_job_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "location")]
    pub fn connect_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_location_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "paused")]
    pub fn connect_paused_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paused_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::paused\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paused_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state-message")]
    pub fn connect_state_message_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_message_trampoline<F: Fn(&Printer) + 'static>(
            this: *mut ffi::GtkPrinter,
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
                b"notify::state-message\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_message_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Printer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Printer`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct PrinterBuilder {
    accepts_pdf: Option<bool>,
    accepts_ps: Option<bool>,
    is_virtual: Option<bool>,
    name: Option<String>,
}

impl PrinterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PrinterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Printer`].
    pub fn build(self) -> Printer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref accepts_pdf) = self.accepts_pdf {
            properties.push(("accepts-pdf", accepts_pdf));
        }
        if let Some(ref accepts_ps) = self.accepts_ps {
            properties.push(("accepts-ps", accepts_ps));
        }
        if let Some(ref is_virtual) = self.is_virtual {
            properties.push(("is-virtual", is_virtual));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        glib::Object::new::<Printer>(&properties).expect("Failed to create an instance of Printer")
    }

    pub fn accepts_pdf(mut self, accepts_pdf: bool) -> Self {
        self.accepts_pdf = Some(accepts_pdf);
        self
    }

    pub fn accepts_ps(mut self, accepts_ps: bool) -> Self {
        self.accepts_ps = Some(accepts_ps);
        self
    }

    pub fn is_virtual(mut self, is_virtual: bool) -> Self {
        self.is_virtual = Some(is_virtual);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}