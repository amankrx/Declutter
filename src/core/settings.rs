/* settings.rs
 *
 * Copyright 2022-2023 Aman Kumar <akumar@gnome.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::{core::UnitSystem, settings_getter_setter};
use gtk::{gio, glib, prelude::*};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone)]
pub struct Settings(gio::Settings);

static mut SETTINGS: Option<Settings> = None;

impl Settings {
    settings_getter_setter!(i32, window_height, "window-height");
    settings_getter_setter!(bool, window_is_maximized, "window-is-maximized");
    settings_getter_setter!(i32, window_width, "window-width");
    settings_getter_setter!(u32, active_user_id, "active-user-id");

    delegate::delegate! {
        to self.0 {
            pub fn bind<'a, P: IsA<glib::Object>>(
                &'a self,
                key: &'a str,
                object: &'a P,
                property: &'a str
            ) -> gio::BindingBuilder<'a>;

            pub fn connect_changed<F: Fn(&gio::Settings, &str) + 'static>(
                &self,
                detail: Option<&str>,
                f: F
            ) -> glib::SignalHandlerId;

            pub fn disconnect(&self, handler_id: glib::SignalHandlerId);

            fn enum_(&self, key: &str) -> i32;

            fn get<U: glib::FromVariant>(&self, key: &str) -> U;

            fn set<U: ToVariant>(&self, key: &str, value: &U) -> Result<(), glib::BoolError>;

            fn set_enum(&self, key: &str, value: i32) -> Result<(), glib::BoolError>;

            fn set_strv(&self, key: &str, value: &[&str]) -> Result<(), glib::BoolError>;

            fn set_string(&self, key: &str, value: &str) -> Result<(), glib::BoolError>;

            fn string(&self, key: &str) -> glib::GString;

            fn strv(&self, key: &str) -> Vec<glib::GString>;
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::instance()
    }
}

impl Settings {
    pub fn instance() -> Self {
        unsafe {
            SETTINGS.as_ref().map_or_else(
                || {
                    let settings = Self(gio::Settings::new("com.amankrx.Declutter"));
                    SETTINGS = Some(settings.clone());
                    settings
                },
                std::clone::Clone::clone,
            )
        }
    }

    /// Connect to the `unitsystem` key changing. Keep in mind that the key has to be read once before connecting or this won't do anything!
    pub fn connect_unit_system_changed<F: Fn(&gio::Settings, &str) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_changed(Some("unitsystem"), move |s, name| {
            f(s, name);
        })
    }

    /// Get the current unit system.
    pub fn unit_system(&self) -> UnitSystem {
        UnitSystem::from_i32(self.enum_("unitsystem")).unwrap()
    }

    /// Set the current unit system.
    pub fn set_unit_system(&self, value: UnitSystem) {
        self.set_enum("unitsystem", value.to_i32().unwrap())
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::Settings;
    use crate::core::utils::init_gschema;

    fn get() -> (Option<tempfile::TempDir>, Settings) {
        (init_gschema(), Settings::instance())
    }

    #[test]
    fn window_height() {
        let (_tmp, settings) = get();
        settings.set_window_height(settings.window_height());
    }

    #[test]
    fn window_is_maximized() {
        let (_tmp, settings) = get();
        settings.set_window_is_maximized(settings.window_is_maximized());
    }

    #[test]
    fn window_width() {
        let (_tmp, settings) = get();
        settings.set_window_width(settings.window_width());
    }

    #[test]
    fn unit_system() {
        let (_tmp, settings) = get();
        settings.set_unit_system(settings.unit_system());
    }
}
