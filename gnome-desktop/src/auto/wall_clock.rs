// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    ///
    ///
    /// # Implements
    ///
    /// [`WallClockExt`][trait@crate::prelude::WallClockExt]
    #[doc(alias = "GnomeWallClock")]
    pub struct WallClock(Object<ffi::GnomeWallClock, ffi::GnomeWallClockClass>);

    match fn {
        type_ => || ffi::gnome_wall_clock_get_type(),
    }
}

impl WallClock {
    pub const NONE: Option<&'static WallClock> = None;

    /// Creates a new #GnomeWallClock
    ///
    /// # Returns
    ///
    /// the new clock
    #[doc(alias = "gnome_wall_clock_new")]
    pub fn new() -> WallClock {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gnome_wall_clock_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WallClock`] objects.
    ///
    /// This method returns an instance of [`WallClockBuilder`](crate::builders::WallClockBuilder) which can be used to create [`WallClock`] objects.
    pub fn builder() -> WallClockBuilder {
        WallClockBuilder::default()
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WallClock`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WallClockBuilder {
    time_only: Option<bool>,
}

impl WallClockBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`WallClockBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WallClock`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WallClock {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref time_only) = self.time_only {
            properties.push(("time-only", time_only));
        }
        glib::Object::new::<WallClock>(&properties)
    }

    /// If [`true`], the formatted clock will never include a date or the
    /// day of the week, irrespective of configuration.
    pub fn time_only(mut self, time_only: bool) -> Self {
        self.time_only = Some(time_only);
        self
    }
}

/// Trait containing all [`struct@WallClock`] methods.
///
/// # Implementors
///
/// [`WallClock`][struct@crate::WallClock]
pub trait WallClockExt: 'static {
    /// Returns the string representing the current time of this clock
    /// according to the user settings.
    ///
    /// # Returns
    ///
    /// the time of the clock as a string.
    ///      This string points to internally allocated storage and
    ///      must not be freed, modified or stored.
    #[doc(alias = "gnome_wall_clock_get_clock")]
    #[doc(alias = "get_clock")]
    fn clock(&self) -> Option<glib::GString>;

    //#[doc(alias = "gnome_wall_clock_get_timezone")]
    //#[doc(alias = "get_timezone")]
    //fn timezone(&self) -> /*Ignored*/Option<glib::TimeZone>;

    //#[doc(alias = "gnome_wall_clock_string_for_datetime")]
    //fn string_for_datetime(&self, now: /*Ignored*/&glib::DateTime, clock_format: /*Ignored*/gdesktop_enums::ClockFormat, show_weekday: bool, show_full_date: bool, show_seconds: bool) -> Option<glib::GString>;

    /// If [`true`], the formatted clock will never include a date or the
    /// day of the week, irrespective of configuration.
    #[doc(alias = "time-only")]
    fn is_time_only(&self) -> bool;

    /// If [`true`], the formatted clock will never include a date or the
    /// day of the week, irrespective of configuration.
    #[doc(alias = "time-only")]
    fn set_time_only(&self, time_only: bool);

    #[doc(alias = "clock")]
    fn connect_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "time-only")]
    fn connect_time_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timezone")]
    fn connect_timezone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WallClock>> WallClockExt for O {
    fn clock(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gnome_wall_clock_get_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn timezone(&self) -> /*Ignored*/Option<glib::TimeZone> {
    //    unsafe { TODO: call ffi:gnome_wall_clock_get_timezone() }
    //}

    //fn string_for_datetime(&self, now: /*Ignored*/&glib::DateTime, clock_format: /*Ignored*/gdesktop_enums::ClockFormat, show_weekday: bool, show_full_date: bool, show_seconds: bool) -> Option<glib::GString> {
    //    unsafe { TODO: call ffi:gnome_wall_clock_string_for_datetime() }
    //}

    fn is_time_only(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "time-only")
    }

    fn set_time_only(&self, time_only: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "time-only", &time_only)
    }

    fn connect_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_trampoline<P: IsA<WallClock>, F: Fn(&P) + 'static>(
            this: *mut ffi::GnomeWallClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WallClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_time_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_time_only_trampoline<P: IsA<WallClock>, F: Fn(&P) + 'static>(
            this: *mut ffi::GnomeWallClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WallClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::time-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_time_only_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timezone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timezone_trampoline<P: IsA<WallClock>, F: Fn(&P) + 'static>(
            this: *mut ffi::GnomeWallClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WallClock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timezone\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timezone_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WallClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WallClock")
    }
}