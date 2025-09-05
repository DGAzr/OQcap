use gtk4::prelude::*;
use gtk4::{glib, Box, Button, TextView, TextBuffer, ScrolledWindow};
use libadwaita::prelude::*;
use libadwaita::{Application as AdwApplication, ApplicationWindow as AdwApplicationWindow};
use std::process::Command;

mod config;
use config::Config;

const APP_ID: &str = "com.github.oqcap";

fn main() -> glib::ExitCode {
    let app = AdwApplication::builder()
        .application_id(APP_ID)
        .build();

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config: {}. Using defaults.", e);
        Config::default()
    });

    app.connect_activate(move |app| build_ui(app, config.clone()));
    app.run()
}

fn build_ui(app: &AdwApplication, config: Config) {
    // Create the main window
    let window = AdwApplicationWindow::builder()
        .application(app)
        .title("Quick Capture")
        .default_width(400)
        .default_height(300)
        .resizable(true)
        .build();




    // Create main content box
    let content_box = Box::new(gtk4::Orientation::Vertical, 12);
    content_box.set_margin_top(12);
    content_box.set_margin_bottom(12);
    content_box.set_margin_start(12);
    content_box.set_margin_end(12);

    // Create text view for input
    let text_buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);
    let text_view = TextView::builder()
        .buffer(&text_buffer)
        .wrap_mode(gtk4::WrapMode::Word)
        .accepts_tab(false)
        .build();

    // TextView doesn't have placeholder text, so we'll add a subtle hint
    text_view.set_monospace(false);

    // Create a placeholder label that disappears when user types
    let placeholder_label = gtk4::Label::builder()
        .label("Enter your text here...")
        .css_classes(vec!["dim-label".to_string()])
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Start)
        .margin_top(8)
        .margin_start(8)
        .build();

    // Create overlay to show placeholder over text view
    let overlay = gtk4::Overlay::new();
    overlay.set_child(Some(&text_view));
    overlay.add_overlay(&placeholder_label);

    // Hide placeholder when text is entered
    let placeholder_clone = placeholder_label.clone();
    text_buffer.connect_changed(move |buffer| {
        let has_text = buffer.char_count() > 0;
        placeholder_clone.set_visible(!has_text);
    });

    // Create scrolled window for the overlay (which contains text view + placeholder)
    let scrolled_window = ScrolledWindow::builder()
        .child(&overlay)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .has_frame(true)
        .vexpand(true)
        .build();

    // Create submit button
    let submit_button = Button::builder()
        .label("Send to Obsidian")
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    // Create button box for proper alignment
    let button_box = Box::new(gtk4::Orientation::Horizontal, 6);
    button_box.set_halign(gtk4::Align::End);
    button_box.append(&submit_button);

    // Add widgets to content box
    content_box.append(&scrolled_window);
    content_box.append(&button_box);

    // Set up the window content
    window.set_content(Some(&content_box));

    // Clone references for closures
    let window_clone = window.clone();
    let text_buffer_clone = text_buffer.clone();

    // Connect submit button
    submit_button.connect_clicked(move |_| {
        let start_iter = text_buffer_clone.start_iter();
        let end_iter = text_buffer_clone.end_iter();
        let text = text_buffer_clone.text(&start_iter, &end_iter, false);
        
        if !text.trim().is_empty() {
            send_to_obsidian(&text, &config);
            window_clone.close();
        }
    });

    // Set up keyboard shortcuts - attach to text view for better focus handling
    let window_clone2 = window.clone();
    let submit_button_clone = submit_button.clone();
    
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, modifier| {
        // Ctrl+Enter to submit
        if key == gtk4::gdk::Key::Return && modifier.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
            submit_button_clone.emit_clicked();
            return gtk4::glib::Propagation::Stop;
        }
        
        // Escape to close
        if key == gtk4::gdk::Key::Escape {
            window_clone2.close();
            return gtk4::glib::Propagation::Stop;
        }
        
        gtk4::glib::Propagation::Proceed
    });
    
    // Attach key controller to text view instead of window for better focus handling
    text_view.add_controller(key_controller);

    // Focus the text view on startup
    text_view.grab_focus();

    // Show the window
    window.present();
}

fn send_to_obsidian(text: &str, config: &Config) {
    // Build the Obsidian URL using configuration
    let obsidian_url = config.build_obsidian_url(text);
    
    // Use xdg-open to open the URL
    let result = Command::new("xdg-open")
        .arg(&obsidian_url)
        .spawn();
    
    match result {
        Ok(_) => println!("Successfully sent text to Obsidian: {}", obsidian_url),
        Err(e) => eprintln!("Failed to send text to Obsidian: {}", e),
    }
}

