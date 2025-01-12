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

use adw::prelude::AdwDialogExt;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::ShutterWindow;
use crate::config::VERSION;

mod imp {
	use super::*;

	#[derive(Debug, Default)]
	pub struct ShutterApplication {}

	#[glib::object_subclass]
	impl ObjectSubclass for ShutterApplication {
		type ParentType = adw::Application;
		type Type = super::ShutterApplication;

		const NAME: &'static str = "ShutterApplication";
	}

	impl ObjectImpl for ShutterApplication {
		fn constructed(&self) {
			self.parent_constructed();
			let obj = self.obj();
			obj.setup_gactions();
			obj.set_accels_for_action("app.quit", &["<primary>q"]);
		}
	}

	impl ApplicationImpl for ShutterApplication {
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
				let window = ShutterWindow::new(&*application);
				window.upcast()
			};

			// Ask the window manager/compositor to present the window
			window.present();
		}
	}

	impl GtkApplicationImpl for ShutterApplication {}
	impl AdwApplicationImpl for ShutterApplication {}
}

glib::wrapper! {
	pub struct ShutterApplication(ObjectSubclass<imp::ShutterApplication>)
		@extends gio::Application, gtk::Application, adw::Application,
		@implements gio::ActionGroup, gio::ActionMap;
}

impl ShutterApplication {
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
		let about = adw::AboutDialog::builder()
			.application_icon("shutter")
			// TODO remove extra icons
			.application_icon("org.shutter-project.Shutter")
			.application_icon("shutter")
			.version(VERSION)
			.website("https://shutter-project.org")
			.issue_url("https://github.com/shutter-project/shutter/issues")
			.license_type(gtk::License::Gpl30)
			.artists(
				String::from_utf8(
					gio::resources_lookup_data(
						"/org/shutter-project/Shutter/credits/art",
						gio::ResourceLookupFlags::empty(),
					)
					.expect("no artist data")
					.to_vec(),
				)
				.expect("can't convert artists to utf8")
				.lines()
				.collect::<Vec<_>>(),
			)
			.developers(
				String::from_utf8(
					gio::resources_lookup_data(
						"/org/shutter-project/Shutter/credits/dev",
						gio::ResourceLookupFlags::empty(),
					)
					.expect("no dev data")
					.to_vec(),
				)
				.expect("can't convert devs to utf8")
				.lines()
				.collect::<Vec<_>>(),
			)
			.copyright(
				String::from_utf8(
					gio::resources_lookup_data(
						"/org/shutter-project/Shutter/credits/copyright",
						gio::ResourceLookupFlags::empty(),
					)
					.expect("no copyright data")
					.to_vec(),
				)
				.expect("can't convert copyright to utf8"),
			)
			.build();

		about.present(Some(&window));
	}
}
