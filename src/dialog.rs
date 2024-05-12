use crate::{image, mastodon, text, twitter, MessageKind, ServiceKind};
use egg_mode_text;
use gdk;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkApplicationExt};
use glib;
use gtk;
use gtk::prelude::{
    ButtonExt, GtkWindowExt, HeaderBarExt, LabelExt, TextBufferExt,
    WidgetExt, BuilderExtManual, WidgetExtManual,
};
use gtk::{
    TextBuffer
};
use std::env;
use gtk::gio::ApplicationFlags;

// Constants for Character count
const URL_COUNT: i32 = 23;
const TWITTER_COUNT: i32 = 280;
const MASTODON_COUNT: i32 = 500;
const TWITTER_IMAGE_COUNT: i32 = TWITTER_COUNT - URL_COUNT;

// Used to bypass Rust's borrowing and ownership security features
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}
pub fn dialog(service: ServiceKind, message: MessageKind) {
    let application =
        gtk::Application::new(Some("pet.kate.rustgrab"), ApplicationFlags::NON_UNIQUE);
    glib::set_prgname(Some("rustgrab"));
    application.connect_startup(move |app| {
        // Creates variables for objects in Glade GTK file
        let mut builder = gtk::Builder::new();
        builder.add_from_string(include_str!("../data/gtk/dialog.ui")).expect("Failed to include dialog");
        let window: gtk::Window = builder.object("window").unwrap();
        let header: gtk::HeaderBar = builder.object("header").unwrap();
        let text: gtk::TextView = builder.object("text").unwrap();
        let buffer: gtk::TextBuffer = builder.object("buffer").unwrap();
        let cancel: gtk::Button = builder.object("cancel").unwrap();
        let send: gtk::Button = builder.object("send").unwrap();
        let image: gtk::Button = builder.object("image").unwrap();
        let count: gtk::Label = builder.object("count").unwrap();
        window.set_application(Some(app));

        // Set Headerbar Subtitle and Default Character Count Label
        match service {
            ServiceKind::Twitter => {
                header.set_subtitle(Some("Twitter"));
                match message {
                    MessageKind::Image => count.set_label(&TWITTER_IMAGE_COUNT.to_string()),
                    MessageKind::Text => count.set_label(&TWITTER_COUNT.to_string()),
                };
            }
            ServiceKind::Mastodon => {
                header.set_subtitle(Some("Mastodon"));
                count.set_label(&MASTODON_COUNT.to_string());
            }
            ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
        }

        // If user is not sending an image, then remove view screenshot button, and disallow user to send
        if let MessageKind::Text = message {
            unsafe{ image.destroy(); }
            send.set_sensitive(false);
        }

        // Opens screenshot made by user
        image.connect_clicked(move |_| {
            image::open_temp();
            return;
        });

        // When window deleted by user (closed), quit GTK and delete temporary image if possible
        window.connect_delete_event(move |window, _| {
            unsafe{ window.destroy(); }
            if let MessageKind::Image = message {
                image::delete_temp();
            }
            glib::Propagation::Stop
        });

        // When cancel button clicked, quit GTK and delete temporary image if possible
        cancel.connect_clicked(clone!(window => move |_| {
            unsafe{window.destroy();}
            if let MessageKind::Image = message {
                image::delete_temp();
            };
        }));

        // When send button clicked
        send.connect_clicked(clone!(buffer,window => move |_| {
            // Get String from Text Buffer (text entered by user)
            let status: String = format!("{}", buffer);
            /*let status: String = match TextBuffer::get_text(
                &buffer,
                &TextBuffer::get_start_iter(&buffer),
                &TextBuffer::get_end_iter(&buffer),
                false,
            ) {
                Some(string) => string,
                None => String::new()
            };*/

            // Checks if Twitter or Mastodon,
            // then checks if status is being sent with an image or not,
            // then decides what to do with the status
            // Creates thread to be able to close the GTK window while sending status/image
            match service {
                ServiceKind::Twitter => match message {
                    MessageKind::Image => {
                            glib::idle_add(move || {
                                twitter::image(status.clone());
                                glib::ControlFlow::Continue
                            });
                        unsafe{window.destroy();}
                    }
                    MessageKind::Text => {
                        if !status.is_empty() {
                                glib::idle_add(move || {
                                    twitter::tweet(status.clone());
                                    glib::ControlFlow::Continue
                                });
                            unsafe{window.destroy();}
                        }
                    }
                },
                ServiceKind::Mastodon => match message {
                    MessageKind::Image => {
                            glib::idle_add(move || {
                                mastodon::image(status.clone());
                                glib::ControlFlow::Continue
                            });
                        unsafe{window.destroy();}
                    }
                    MessageKind::Text => {
                        if !status.is_empty() {
                                glib::idle_add(move || {
                                    mastodon::toot(status.clone());
                                    glib::ControlFlow::Continue
                                });
                            unsafe{window.destroy();}
                        }
                    }
                },
                ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
            }
        }));

        // Character count when keys are pressed in the Text Box
        text.connect_key_release_event(clone!(send,count => move |_, _| {
        let status: String = format!("{}", buffer);
        let status_count = char_count(service, status, message);

        // uses markdown to set color
        let mut limit = format!("<span foreground=\"#EE0456\">");
        limit.push_str(&status_count.to_string());
        limit.push_str("</span>");
        let mut hit = format!("<span foreground=\"#ECA60B\">");
        hit.push_str(&status_count.to_string());
        hit.push_str("</span>");

        match message {
            MessageKind::Image => match service {
                ServiceKind::Twitter => {
                    if status_count <= 20 && status_count >= 0 {
                        count.set_markup(&hit);
                    } else if status_count < 0 {
                        count.set_markup(&limit);
                    } else {
                        count.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Mastodon => {
                    if status_count < 0 {
                        count.set_markup(&limit);
                    } else {
                        count.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
            },
            MessageKind::Text => match service {
                ServiceKind::Twitter => {
                    if status_count >= TWITTER_COUNT || status_count < 0 {
                        send.set_sensitive(false);
                    } else {
                        send.set_sensitive(true);
                    }
                    if status_count <= 20 && status_count >= 0 {
                        count.set_markup(&hit);
                    } else if status_count < 0 {
                        count.set_markup(&limit);
                    } else {
                        count.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Mastodon => {
                    if status_count >= MASTODON_COUNT || status_count < 0 {
                        send.set_sensitive(false);
                    } else {
                        send.set_sensitive(true);
                    }
                    if status_count < 0 {
                        count.set_markup(&limit);
                    } else {
                        count.set_label(&status_count.to_string());
                    }
                }
                ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
            },
        };
        glib::Propagation::Proceed
        }));

        // Enables CTRL+Enter shortcut to send a status
        text.connect_key_press_event(clone!(send => move |_, key| {

            if key.state().intersects(gdk::ModifierType::CONTROL_MASK) &&
                                            key.keyval() == gdk::keys::constants::Return &&
                                            send.get_sensitive() {
                            send.clicked();
            };
            glib::Propagation::Proceed
        }));

        // Shows the window created
        window.show_all();
    });
    application.connect_activate(|_| {});
    let mut args: Vec<String> = env::args().collect();
    for i in 0..args.len() - 1 {
        if args[i] == "-f".to_string() {
            args.remove(i);
        }
    }
    application.run_with_args(&args);
}

// Character count using egg_mode_text as a representation of Twitter's character count
fn char_count(service: ServiceKind, status: String, message: MessageKind) -> i32 {
    let remaining = match service {
        ServiceKind::Twitter => match message {
            MessageKind::Image => {
                TWITTER_IMAGE_COUNT
                    - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
            }
            MessageKind::Text => {
                TWITTER_COUNT - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
            }
        },
        ServiceKind::Mastodon => {
            MASTODON_COUNT - egg_mode_text::character_count(&status, URL_COUNT, URL_COUNT) as i32
        }
        ServiceKind::Imgur => unreachable!("Imgur does not open a GTK dialog"),
    };
    return remaining;
}
