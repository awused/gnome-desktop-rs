// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GnomeDesktopThumbnailSize = c_int;
pub const GNOME_DESKTOP_THUMBNAIL_SIZE_NORMAL: GnomeDesktopThumbnailSize = 0;
pub const GNOME_DESKTOP_THUMBNAIL_SIZE_LARGE: GnomeDesktopThumbnailSize = 1;
pub const GNOME_DESKTOP_THUMBNAIL_SIZE_XLARGE: GnomeDesktopThumbnailSize = 2;
pub const GNOME_DESKTOP_THUMBNAIL_SIZE_XXLARGE: GnomeDesktopThumbnailSize = 3;

// Constants
pub const GNOME_DESKTOP_PLATFORM_VERSION: c_int = 43;

// Callbacks
pub type GnomeIdleMonitorWatchFunc =
    Option<unsafe extern "C" fn(*mut GnomeIdleMonitor, c_uint, gpointer)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeDesktopThumbnailFactoryClass {
    pub parent: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GnomeDesktopThumbnailFactoryClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeDesktopThumbnailFactoryClass @ {self:p}"))
            .field("parent", &self.parent)
            .finish()
    }
}

#[repr(C)]
pub struct _GnomeDesktopThumbnailFactoryPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GnomeDesktopThumbnailFactoryPrivate = *mut _GnomeDesktopThumbnailFactoryPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeIdleMonitorClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GnomeIdleMonitorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeIdleMonitorClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GnomeIdleMonitorPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GnomeIdleMonitorPrivate = *mut _GnomeIdleMonitorPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomePnpIdsClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GnomePnpIdsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomePnpIdsClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GnomePnpIdsPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GnomePnpIdsPrivate = *mut _GnomePnpIdsPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeWallClockClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GnomeWallClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeWallClockClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GnomeWallClockPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GnomeWallClockPrivate = *mut _GnomeWallClockPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeXkbInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for GnomeXkbInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeXkbInfoClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GnomeXkbInfoPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GnomeXkbInfoPrivate = *mut _GnomeXkbInfoPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeDesktopThumbnailFactory {
    pub parent: gobject::GObject,
    pub priv_: *mut GnomeDesktopThumbnailFactoryPrivate,
}

impl ::std::fmt::Debug for GnomeDesktopThumbnailFactory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeDesktopThumbnailFactory @ {self:p}"))
            .field("parent", &self.parent)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeIdleMonitor {
    pub parent: gobject::GObject,
    pub priv_: *mut GnomeIdleMonitorPrivate,
}

impl ::std::fmt::Debug for GnomeIdleMonitor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeIdleMonitor @ {self:p}"))
            .field("parent", &self.parent)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomePnpIds {
    pub parent: gobject::GObject,
    pub priv_: *mut GnomePnpIdsPrivate,
}

impl ::std::fmt::Debug for GnomePnpIds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomePnpIds @ {self:p}"))
            .field("parent", &self.parent)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeWallClock {
    pub parent_object: gobject::GObject,
    pub priv_: *mut GnomeWallClockPrivate,
}

impl ::std::fmt::Debug for GnomeWallClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeWallClock @ {self:p}"))
            .field("parent_object", &self.parent_object)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GnomeXkbInfo {
    pub parent_object: gobject::GObject,
    pub priv_: *mut GnomeXkbInfoPrivate,
}

impl ::std::fmt::Debug for GnomeXkbInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GnomeXkbInfo @ {self:p}"))
            .field("parent_object", &self.parent_object)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[link(name = "gnome-desktop-4")]
extern "C" {

    //=========================================================================
    // GnomeDesktopThumbnailFactory
    //=========================================================================
    pub fn gnome_desktop_thumbnail_factory_get_type() -> GType;
    pub fn gnome_desktop_thumbnail_factory_new(
        size: GnomeDesktopThumbnailSize,
    ) -> *mut GnomeDesktopThumbnailFactory;
    pub fn gnome_desktop_thumbnail_factory_can_thumbnail(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mime_type: *const c_char,
        mtime: c_long,
    ) -> gboolean;
    pub fn gnome_desktop_thumbnail_factory_create_failed_thumbnail(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mtime: c_long,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gnome_desktop_thumbnail_factory_create_failed_thumbnail_async(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        original_mtime: c_long,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gnome_desktop_thumbnail_factory_create_failed_thumbnail_finish(
        factory: *mut GnomeDesktopThumbnailFactory,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    #[cfg(feature = "v42")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v42")))]
    pub fn gnome_desktop_thumbnail_factory_generate_thumbnail(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mime_type: *const c_char,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn gnome_desktop_thumbnail_factory_generate_thumbnail_async(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mime_type: *const c_char,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gnome_desktop_thumbnail_factory_generate_thumbnail_finish(
        factory: *mut GnomeDesktopThumbnailFactory,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn gnome_desktop_thumbnail_factory_has_valid_failed_thumbnail(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mtime: c_long,
    ) -> gboolean;
    pub fn gnome_desktop_thumbnail_factory_lookup(
        factory: *mut GnomeDesktopThumbnailFactory,
        uri: *const c_char,
        mtime: c_long,
    ) -> *mut c_char;
    pub fn gnome_desktop_thumbnail_factory_save_thumbnail(
        factory: *mut GnomeDesktopThumbnailFactory,
        thumbnail: *mut gdk_pixbuf::GdkPixbuf,
        uri: *const c_char,
        original_mtime: c_long,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gnome_desktop_thumbnail_factory_save_thumbnail_async(
        factory: *mut GnomeDesktopThumbnailFactory,
        thumbnail: *mut gdk_pixbuf::GdkPixbuf,
        uri: *const c_char,
        original_mtime: c_long,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gnome_desktop_thumbnail_factory_save_thumbnail_finish(
        factory: *mut GnomeDesktopThumbnailFactory,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // GnomeIdleMonitor
    //=========================================================================
    pub fn gnome_idle_monitor_get_type() -> GType;
    pub fn gnome_idle_monitor_new() -> *mut GnomeIdleMonitor;
    pub fn gnome_idle_monitor_add_idle_watch(
        monitor: *mut GnomeIdleMonitor,
        interval_msec: u64,
        callback: GnomeIdleMonitorWatchFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn gnome_idle_monitor_add_user_active_watch(
        monitor: *mut GnomeIdleMonitor,
        callback: GnomeIdleMonitorWatchFunc,
        user_data: gpointer,
        notify: glib::GDestroyNotify,
    ) -> c_uint;
    pub fn gnome_idle_monitor_get_idletime(monitor: *mut GnomeIdleMonitor) -> u64;
    pub fn gnome_idle_monitor_remove_watch(monitor: *mut GnomeIdleMonitor, id: c_uint);

    //=========================================================================
    // GnomePnpIds
    //=========================================================================
    pub fn gnome_pnp_ids_get_type() -> GType;
    pub fn gnome_pnp_ids_new() -> *mut GnomePnpIds;
    pub fn gnome_pnp_ids_get_pnp_id(
        pnp_ids: *mut GnomePnpIds,
        pnp_id: *const c_char,
    ) -> *mut c_char;

    //=========================================================================
    // GnomeWallClock
    //=========================================================================
    pub fn gnome_wall_clock_get_type() -> GType;
    pub fn gnome_wall_clock_new() -> *mut GnomeWallClock;
    pub fn gnome_wall_clock_get_clock(clock: *mut GnomeWallClock) -> *const c_char;
    pub fn gnome_wall_clock_get_timezone(clock: *mut GnomeWallClock) -> *mut glib::GTimeZone;
    pub fn gnome_wall_clock_string_for_datetime(
        self_: *mut GnomeWallClock,
        now: *mut glib::GDateTime,
        clock_format: gdesktop_enums::GDesktopClockFormat,
        show_weekday: gboolean,
        show_full_date: gboolean,
        show_seconds: gboolean,
    ) -> *mut c_char;

    //=========================================================================
    // GnomeXkbInfo
    //=========================================================================
    pub fn gnome_xkb_info_get_type() -> GType;
    pub fn gnome_xkb_info_new() -> *mut GnomeXkbInfo;
    pub fn gnome_xkb_info_description_for_group(
        self_: *mut GnomeXkbInfo,
        group_id: *const c_char,
    ) -> *const c_char;
    pub fn gnome_xkb_info_description_for_option(
        self_: *mut GnomeXkbInfo,
        group_id: *const c_char,
        id: *const c_char,
    ) -> *const c_char;
    pub fn gnome_xkb_info_get_all_layouts(self_: *mut GnomeXkbInfo) -> *mut glib::GList;
    pub fn gnome_xkb_info_get_all_option_groups(self_: *mut GnomeXkbInfo) -> *mut glib::GList;
    pub fn gnome_xkb_info_get_languages_for_layout(
        self_: *mut GnomeXkbInfo,
        layout_id: *const c_char,
    ) -> *mut glib::GList;
    pub fn gnome_xkb_info_get_layout_info(
        self_: *mut GnomeXkbInfo,
        id: *const c_char,
        display_name: *mut *const c_char,
        short_name: *mut *const c_char,
        xkb_layout: *mut *const c_char,
        xkb_variant: *mut *const c_char,
    ) -> gboolean;
    pub fn gnome_xkb_info_get_layouts_for_country(
        self_: *mut GnomeXkbInfo,
        country_code: *const c_char,
    ) -> *mut glib::GList;
    pub fn gnome_xkb_info_get_layouts_for_language(
        self_: *mut GnomeXkbInfo,
        language_code: *const c_char,
    ) -> *mut glib::GList;
    pub fn gnome_xkb_info_get_options_for_group(
        self_: *mut GnomeXkbInfo,
        group_id: *const c_char,
    ) -> *mut glib::GList;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gnome_desktop_thumbnail_is_valid(
        pixbuf: *mut gdk_pixbuf::GdkPixbuf,
        uri: *const c_char,
        mtime: c_long,
    ) -> gboolean;
    pub fn gnome_desktop_thumbnail_path_for_uri(
        uri: *const c_char,
        size: GnomeDesktopThumbnailSize,
    ) -> *mut c_char;
    pub fn gnome_get_all_locales() -> *mut *mut c_char;
    pub fn gnome_get_country_from_code(
        code: *const c_char,
        translation: *const c_char,
    ) -> *mut c_char;
    pub fn gnome_get_country_from_locale(
        locale: *const c_char,
        translation: *const c_char,
    ) -> *mut c_char;
    pub fn gnome_get_input_source_from_locale(
        locale: *const c_char,
        type_: *mut *const c_char,
        id: *mut *const c_char,
    ) -> gboolean;
    pub fn gnome_get_language_from_code(
        code: *const c_char,
        translation: *const c_char,
    ) -> *mut c_char;
    pub fn gnome_get_language_from_locale(
        locale: *const c_char,
        translation: *const c_char,
    ) -> *mut c_char;
    #[cfg(feature = "v43")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v43")))]
    pub fn gnome_get_platform_version() -> c_int;
    pub fn gnome_get_translated_modifier(
        modifier: *const c_char,
        translation: *const c_char,
    ) -> *mut c_char;
    pub fn gnome_language_has_translations(code: *const c_char) -> gboolean;
    pub fn gnome_normalize_locale(locale: *const c_char) -> *mut c_char;
    pub fn gnome_parse_locale(
        locale: *const c_char,
        language_codep: *mut *mut c_char,
        country_codep: *mut *mut c_char,
        codesetp: *mut *mut c_char,
        modifierp: *mut *mut c_char,
    ) -> gboolean;
    pub fn gnome_start_systemd_scope(
        name: *const c_char,
        pid: i32,
        description: *const c_char,
        connection: *mut gio::GDBusConnection,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gnome_start_systemd_scope_finish(
        res: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;

}
