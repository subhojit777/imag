//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::error::Error;

use notificator::Notificator;
use self::err::*;
use self::ok::*;

pub mod err {
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::error::Error;

    use notify_rust::Notification as RustNotification;
    use notify_rust::NotificationUrgency;

    use error::ResultExt;
    use error::NotificationErrorKind as NEK;
    use notificator::default::Urgency;
    use notificator::default::Notification;
    use notificator::Notificator;
    use error::Result;

    #[derive(Debug, Default, Clone)]
    pub struct ErrorNotification(Notification, usize);

    impl ErrorNotification {
        pub fn new(trace: usize, timeout: i32) -> ErrorNotification {
            let notif = Notification {
                timeout,
                message: String::new(), // Not used in this special case
                summary: "[Error]".to_owned(),
                urgency: Urgency::High,
            };

            ErrorNotification(notif, trace)
        }
    }

    impl<T: Error> Notificator<T> for ErrorNotification {

        /// A default implementation for all Types that implement Display
        fn notify(&self, item: &T) -> Result<()>{
            fn trace_notify(urgency: NotificationUrgency, e: &Error, u: usize) -> Result<()> {
                let mut n = RustNotification::new();
                n.appname("imag");
                n.summary("[Error]");
                n.urgency(urgency.clone());
                n.body(e.description());
                try!(n.finalize().show().map(|_| ()).chain_err(|| NEK::Unknown));

                if u > 0 {
                    e.cause().map(|cause| trace_notify(urgency, cause, u - 1));
                }

                Ok(())
            }

            trace_notify(self.0.urgency.clone().into(), item, self.1)
        }

    }

    impl Deref for ErrorNotification {
        type Target = Notification;

        fn deref(&self) -> &Notification {
            &self.0
        }

    }

    impl DerefMut for ErrorNotification {

        fn deref_mut(&mut self) -> &mut Notification {
            &mut self.0
        }

    }

}

pub mod ok {
    use std::ops::Deref;
    use std::ops::DerefMut;

    use notify_rust::Notification as RustNotification;

    use notificator::default::Notification;
    use notificator::Notificator;
    use error::Result;

    #[derive(Debug, Default, Clone)]
    pub struct OkNotification(Notification);

    impl From<Notification> for OkNotification {

        fn from(n: Notification) -> OkNotification {
            OkNotification(n)
        }

    }

    impl<T> Notificator<T> for OkNotification {

        /// A default implementation for all Types that implement Display
        fn notify(&self, _: &T) -> Result<()> {
            let mut n = RustNotification::new();
            n.appname("imag");
            n.summary("[Ok]");
            n.urgency(self.0.urgency.clone().into());
            n.body(&"< >".to_owned());
            n.finalize().show().map(|_| ())?;
            Ok(())
        }

    }

    impl Deref for OkNotification {
        type Target = Notification;

        fn deref(&self) -> &Notification {
            &self.0
        }

    }

    impl DerefMut for OkNotification {

        fn deref_mut(&mut self) -> &mut Notification {
            &mut self.0
        }

    }

}

/// An extension trait for Result types
///
/// Can be used to notify on error or on "Ok(_)" values.
///
/// # Warning
///
/// As the notification could go wrong, but inside a mapping function, the error cannot be given to
/// someone, we ignore errors in the Notificator::notify() call.
pub trait ResultNotification<T, E> {

    /// Notify with a custom Notificator, only notify on Ok(T)
    fn notify_with(self, n: &Notificator<T>) -> Self;

    /// Notify with the OkNotification::default(), only notify on Ok(T)
    fn notify(self) -> Self;

    /// Notify with a custom Notificator, only notify on Err(E)
    fn notify_on_err_with(self, n: &Notificator<E>) -> Self;

    /// Notify with the ErrorNotification::default(), only notify on Err(E)
    fn notify_on_err(self) -> Self;

}

impl<T, E: Error> ResultNotification<T, E> for Result<T, E> {

    fn notify_with(self, n: &Notificator<T>) -> Self {
        self.map(|item| { let _ = n.notify(&item); item })
    }

    fn notify(self) -> Self {
        self.notify_with(&OkNotification::default())
    }

    fn notify_on_err_with(self, n: &Notificator<E>) -> Self {
        self.map_err(|e| { let _ = n.notify(&e); e })
    }

    fn notify_on_err(self) -> Self {
        self.notify_on_err_with(&ErrorNotification::default())
    }

}

