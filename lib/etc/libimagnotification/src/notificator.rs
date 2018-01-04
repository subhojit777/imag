//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use error::Result;

/// A Notificator provides a function that can be called to notify about a certain object.
///
/// # TODO
///
/// The user of the library does _not_ get access to the notification handle.
/// This is not optimal, but enough for today.
///
pub trait Notificator<T> {
    fn notify(&self, item: &T) -> Result<()>;
}

pub mod default {
    use std::fmt::Debug;
    use std::fmt::Display;

    use error::Result;

    use notify_rust::Notification as RustNotification;
    use notify_rust::NotificationUrgency;

    use super::Notificator;

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum Urgency {
        Low,
        Normal,
        High
    }

    impl Default for Urgency {
        fn default() -> Urgency {
            Urgency::Normal
        }
    }

    impl Into<NotificationUrgency> for Urgency {

        fn into(self) -> NotificationUrgency {
            match self {
                Urgency::Low    => NotificationUrgency::Low,
                Urgency::Normal => NotificationUrgency::Normal,
                Urgency::High   => NotificationUrgency::Critical,
            }
        }

    }

    #[derive(Debug, Default, Clone)]
    pub struct Notification {
        pub timeout: i32,
        pub message: String,
        pub summary: String,
        pub urgency: Urgency,
    }

    impl<T: Display> Notificator<T> for Notification {

        /// A default implementation for all Types that implement Display
        fn notify(&self, item: &T) -> Result<()> {
            let mut n = RustNotification::new();
            n.appname("imag");
            n.summary(&self.summary);
            n.urgency(self.urgency.clone().into());
            n.body(&format!("{}: {}", &self.message, item));
            let _ = n.finalize().show(); // Ignoring error here
            Ok(())
        }

    }

    #[derive(Debug, Default, Clone)]
    pub struct DebugNotification(Notification);

    impl From<Notification> for DebugNotification {
        fn from(n: Notification) -> DebugNotification {
            DebugNotification(n)
        }
    }

    impl<T: Debug> Notificator<T> for DebugNotification {

        /// A default implementation for all Types that implement Display
        fn notify(&self, item: &T) -> Result<()> {
            let mut n = RustNotification::new();
            n.appname("imag");
            n.summary(&self.0.summary);
            n.urgency(self.0.urgency.clone().into());
            n.body(&format!("{}: {:?}", &self.0.message, item));
            let _ = n.finalize().show(); // Ignoring error here
            Ok(())
        }

    }

}

