// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::CellRenderer;
use crate::CellRendererMode;
use crate::IconSize;
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
    #[doc(alias = "GtkCellRendererPixbuf")]
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf>) @extends CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    #[doc(alias = "gtk_cell_renderer_pixbuf_new")]
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererPixbuf`] objects.
    ///
    /// This method returns an instance of [`CellRendererPixbufBuilder`] which can be used to create [`CellRendererPixbuf`] objects.
    pub fn builder() -> CellRendererPixbufBuilder {
        CellRendererPixbufBuilder::default()
    }

    pub fn gicon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = glib::Value::from_type(<gio::Icon as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"gicon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gicon` getter")
        }
    }

    pub fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"gicon\0".as_ptr() as *const _,
                gicon.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-name` getter")
        }
    }

    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-name\0".as_ptr() as *const _,
                icon_name.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "icon-size")]
    pub fn icon_size(&self) -> IconSize {
        unsafe {
            let mut value = glib::Value::from_type(<IconSize as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-size` getter")
        }
    }

    #[doc(alias = "icon-size")]
    pub fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"icon-size\0".as_ptr() as *const _,
                icon_size.to_value().to_glib_none().0,
            );
        }
    }

    pub fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pixbuf\0".as_ptr() as *const _,
                pixbuf.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pixbuf-expander-closed\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf-expander-closed` getter")
        }
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn set_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pixbuf-expander-closed\0".as_ptr() as *const _,
                pixbuf_expander_closed.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pixbuf-expander-open\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf-expander-open` getter")
        }
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn set_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pixbuf-expander-open\0".as_ptr() as *const _,
                pixbuf_expander_open.to_value().to_glib_none().0,
            );
        }
    }

    pub fn texture(&self) -> Option<gdk::Texture> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::Texture as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"texture\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `texture` getter")
        }
    }

    pub fn set_texture<P: IsA<gdk::Texture>>(&self, texture: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"texture\0".as_ptr() as *const _,
                texture.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gicon")]
    pub fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf")]
    pub fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn connect_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<
            F: Fn(&CellRendererPixbuf) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::pixbuf-expander-closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_expander_closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn connect_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<
            F: Fn(&CellRendererPixbuf) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::pixbuf-expander-open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_expander_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "texture")]
    pub fn connect_texture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_texture_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                b"notify::texture\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_texture_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererPixbuf`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct CellRendererPixbufBuilder {
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    icon_size: Option<IconSize>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    pixbuf_expander_closed: Option<gdk_pixbuf::Pixbuf>,
    pixbuf_expander_open: Option<gdk_pixbuf::Pixbuf>,
    texture: Option<gdk::Texture>,
    cell_background: Option<String>,
    cell_background_rgba: Option<gdk::RGBA>,
    cell_background_set: Option<bool>,
    height: Option<i32>,
    is_expanded: Option<bool>,
    is_expander: Option<bool>,
    mode: Option<CellRendererMode>,
    sensitive: Option<bool>,
    visible: Option<bool>,
    width: Option<i32>,
    xalign: Option<f32>,
    xpad: Option<u32>,
    yalign: Option<f32>,
    ypad: Option<u32>,
}

impl CellRendererPixbufBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CellRendererPixbufBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererPixbuf`].
    pub fn build(self) -> CellRendererPixbuf {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        if let Some(ref pixbuf_expander_closed) = self.pixbuf_expander_closed {
            properties.push(("pixbuf-expander-closed", pixbuf_expander_closed));
        }
        if let Some(ref pixbuf_expander_open) = self.pixbuf_expander_open {
            properties.push(("pixbuf-expander-open", pixbuf_expander_open));
        }
        if let Some(ref texture) = self.texture {
            properties.push(("texture", texture));
        }
        if let Some(ref cell_background) = self.cell_background {
            properties.push(("cell-background", cell_background));
        }
        if let Some(ref cell_background_rgba) = self.cell_background_rgba {
            properties.push(("cell-background-rgba", cell_background_rgba));
        }
        if let Some(ref cell_background_set) = self.cell_background_set {
            properties.push(("cell-background-set", cell_background_set));
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref is_expanded) = self.is_expanded {
            properties.push(("is-expanded", is_expanded));
        }
        if let Some(ref is_expander) = self.is_expander {
            properties.push(("is-expander", is_expander));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new::<CellRendererPixbuf>(&properties)
            .expect("Failed to create an instance of CellRendererPixbuf")
    }

    pub fn gicon<P: IsA<gio::Icon>>(mut self, gicon: &P) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }

    pub fn pixbuf_expander_closed(mut self, pixbuf_expander_closed: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf_expander_closed = Some(pixbuf_expander_closed.clone());
        self
    }

    pub fn pixbuf_expander_open(mut self, pixbuf_expander_open: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf_expander_open = Some(pixbuf_expander_open.clone());
        self
    }

    pub fn texture<P: IsA<gdk::Texture>>(mut self, texture: &P) -> Self {
        self.texture = Some(texture.clone().upcast());
        self
    }

    pub fn cell_background(mut self, cell_background: &str) -> Self {
        self.cell_background = Some(cell_background.to_string());
        self
    }

    pub fn cell_background_rgba(mut self, cell_background_rgba: &gdk::RGBA) -> Self {
        self.cell_background_rgba = Some(cell_background_rgba.clone());
        self
    }

    pub fn cell_background_set(mut self, cell_background_set: bool) -> Self {
        self.cell_background_set = Some(cell_background_set);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn is_expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    pub fn is_expander(mut self, is_expander: bool) -> Self {
        self.is_expander = Some(is_expander);
        self
    }

    pub fn mode(mut self, mode: CellRendererMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: u32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: u32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

impl fmt::Display for CellRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererPixbuf")
    }
}