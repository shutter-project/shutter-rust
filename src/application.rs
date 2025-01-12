/* application.rs
 *
 * Copyright 2025 Unknown
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::ShutterRustWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ShutterRustApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for ShutterRustApplication {
        const NAME: &'static str = "ShutterRustApplication";
        type Type = super::ShutterRustApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for ShutterRustApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for ShutterRustApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = ShutterRustWindow::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for ShutterRustApplication {}
    }

glib::wrapper! {
    pub struct ShutterRustApplication(ObjectSubclass<imp::ShutterRustApplication>)
        @extends gio::Application, gtk::Application, 
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ShutterRustApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("shutter-rust")
            // TODO remove extra icon
            .logo_icon_name("org.shutter-project.ShutterRust")
            .logo_icon_name("shutter")
            .version(VERSION)
            .authors(vec!["Unknown"])
            .copyright("© 2025 Unknown")
            .build();

        about.present();
    }
}
