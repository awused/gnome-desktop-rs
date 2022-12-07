<!-- file * -->
<!-- fn start_systemd_scope -->
If the current process is running inside a user systemd instance, then move
the launched PID into a transient scope. The given @name will be used to
create a unit name. It should be the application ID or the executable in all
other cases. If a desktop-id is passed then the .desktop suffix will be
stripped.

It is advisable to use this function every time where the started application
can be considered reasonably independent of the launching application. Placing
it in a scope creates proper separation between the programs rather than being
considered a single entity by systemd.

It is always safe to call this function. Note that a successful return code
does not imply that a unit has been created. It solely means that no error
condition was hit sending the request.

If @connection is [`None`] then g_dbus_get() will be called internally.

Note that most callers will not need to handle errors. As such, it is normal
to pass a [`None`] @callback.
## `name`
Name for the application
## `pid`
The PID of the application
## `description`
A description to use for the unit, or [`None`]
## `connection`
An #GDBusConnection to the session bus, or [`None`]
## `cancellable`
#GCancellable to use
## `callback`
Callback to call when the operation is done
<!-- impl WallClockBuilder::fn clock -->
A formatted string representing the current clock display.
<!-- impl WallClockBuilder::fn timezone -->
The timezone used for this clock
<!-- trait WallClockExt::fn timezone -->
Returns the current local time zone used by this clock.

# Returns

the #GTimeZone of the clock.
<!-- trait WallClockExt::fn string_for_datetime -->

# Returns

a newly allocated string representing the date & time
passed, with the options applied.
