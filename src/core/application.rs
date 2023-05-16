/* application.rs
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

use adw::{subclass::prelude::*, AboutWindow};

use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{
    gdk, gio,
    glib::{self, clone, g_error, g_info},
};
use log::{debug, info};

use crate::{
    config::{APP_ID, PKGDATADIR, PROFILE, VERSION},
    core::{i18n, spawn},
    models::User,
    windows::Window,
};

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug)]
    pub struct Application {
        pub settings: gio::Settings,
        pub window: OnceCell<WeakRef<Window>>,
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),
                window: OnceCell::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self) {
            debug!("GtkApplication<Application>::activate");
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }
            let window = Window::new(&app);

            spawn(clone!(@strong app => async move {
                if app.imp().settings.int("active-user-id") <= 0 {
                    match app.create_initital_user() {
                        Ok(()) => {
                            g_info!(APP_ID, "Created initial user...");
                        }
                        Err(e) => {
                            g_error!(APP_ID, "Error creating initial user: {}", e);
                        }
                    }
                }

                app.imp().window
                    .set(window.downgrade())
                    .expect("Window already set.");

                app.main_window().present();
            }));
        }

        fn startup(&self) {
            debug!("GtkApplication<Application>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    fn main_window(&self) -> Window {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_window();
            })
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/amankrx/Declutter/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn create_initital_user(&self) -> Result<(), glib::BoolError> {
        let curr_time = glib::DateTime::now_local()
            .unwrap()
            .format_iso8601()
            .unwrap()
            .to_string();

        let user = User::create("User", &curr_time);
        match user {
            Ok(user) => {
                self.imp()
                    .settings
                    .set_int("active-user-id", user.id() as i32)?;
            }
            Err(e) => {
                println!("Error creating user: {}", e);
            }
        }

        Ok(())
    }

    fn show_about_window(&self) {
        let about_window = AboutWindow::builder()
            .application_icon(APP_ID)
            .license_type(gtk::License::Gpl30)
            .website("https://gitlab.gnome.org/amankrx/declutter/")
            .issue_url("https://gitlab.gnome.org/amankrx/declutter/issues/")
            .comments(&i18n("A Habit Tracking application for GNOME Desktop"))
            .application_name("Declutter")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(gettext("translator-credits"))
            .modal(true)
            .developer_name("Aman Kumar")
            .developers(vec!["Aman Kumar <akumar@gnome.org>"])
            .artists(vec!["Aman Kumar <akumar@gnome.org>"])
            .build();
        about_window.present();
    }

    fn show_preferences_window(&self) {
        todo!()
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("Declutter ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", "/com/amankrx/Declutter/")
            .build()
    }
}
