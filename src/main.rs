// We don't want default methods for all GObject types
#![allow(clippy::new_without_default)]
#![warn(clippy::await_holding_refcell_ref)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::comparison_to_empty)]
#![warn(clippy::manual_find_map)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::option_if_let_else)]
use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

use libdeclutter::{
    config,
    core::{i18n, Application},
};

fn main() -> glib::ExitCode {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(config::GETTEXT_PACKAGE, config::LOCALEDIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(config::GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext(&i18n("Declutter")));
    println!("User data dir: {:?}", glib::user_data_dir());

    let res = gio::Resource::load(config::RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);
    println!(
        "DATABASE_URL=/home/aman/.var/app/com.amankrx.Declutter.Devel/data/declutter.db diesel migration run {:?}",
        gtk::glib::user_data_dir()
    );

    let app = Application::default();
    app.run()
}
