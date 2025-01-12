/* window.rs
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

mod imp {
	use super::*;

	#[derive(Debug, Default, gtk::CompositeTemplate)]
	#[template(resource = "/org/shutter-project/Shutter/window.ui")]
	pub struct ShutterWindow {
		// Template widgets
		//#[template_child]
		//pub header_bar: TemplateChild<gtk::HeaderBar>,
		#[template_child]
		pub label: TemplateChild<gtk::Label>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ShutterWindow {
		type ParentType = gtk::ApplicationWindow;
		type Type = super::ShutterWindow;

		const NAME: &'static str = "ShutterWindow";

		fn class_init(klass: &mut Self::Class) {
			klass.bind_template();
		}

		fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
			obj.init_template();
		}
	}

	impl ObjectImpl for ShutterWindow {}
	impl WidgetImpl for ShutterWindow {}
	impl WindowImpl for ShutterWindow {}
	impl ApplicationWindowImpl for ShutterWindow {}
}

glib::wrapper! {
	pub struct ShutterWindow(ObjectSubclass<imp::ShutterWindow>)
		@extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
		@implements gio::ActionGroup, gio::ActionMap;
}

impl ShutterWindow {
	pub fn new<P: glib::object::IsA<gtk::Application>>(application: &P) -> Self {
		glib::Object::builder()
			.property("application", application)
			.property("show-menubar", true)
			.build()
	}
}
