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

    app.connect_activate(move |app| {
        // Load custom CSS for glass effect after GTK is initialized
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        
        // Apply CSS to default display
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        
        build_ui(app, config.clone())
    });
    
    app.run()
}

fn build_ui(app: &AdwApplication, config: Config) {
    // Create the main window with glass styling
    let window = AdwApplicationWindow::builder()
        .application(app)
        .title("Quick Capture")
        .resizable(true)
        .build();
    
    // Calculate responsive window size (50% width, 10% height, centered)
    if let Some(display) = gtk4::gdk::Display::default() {
        if let Some(monitor) = display.monitors().item(0) {
            if let Some(monitor) = monitor.downcast_ref::<gtk4::gdk::Monitor>() {
                let geometry = monitor.geometry();
                let width = (geometry.width() as f64 * 0.5) as i32;
                let height = (geometry.height() as f64 * 0.1) as i32;
                
                // Set minimum size for usability
                let min_width = 400.max(width);
                let min_height = 120.max(height);
                
                window.set_default_size(min_width, min_height);
                
                // Note: Window centering on Wayland is handled by the compositor
                // and cannot be controlled programmatically for security reasons
            }
        }
    }
    
    // Add glass window styling
    window.add_css_class("glass-window");




    // Create main content box with glass styling (smaller margins for compact window)
    let content_box = Box::new(gtk4::Orientation::Vertical, 12);
    content_box.set_margin_top(16);
    content_box.set_margin_bottom(16);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);
    content_box.add_css_class("glass-content");

    // Create text view for input with glass styling
    let text_buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);
    let text_view = TextView::builder()
        .buffer(&text_buffer)
        .wrap_mode(gtk4::WrapMode::Word)
        .accepts_tab(false)
        .build();

    // Add glass styling to text view
    text_view.add_css_class("glass-textview");
    text_view.set_monospace(false);

    // Create a placeholder label that disappears when user types
    let placeholder_label = gtk4::Label::builder()
        .label("Enter your text here...")
        .css_classes(vec!["glass-placeholder".to_string()])
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Start)
        .margin_top(12)
        .margin_start(12)
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
        .has_frame(false) // Remove default frame for glass effect
        .vexpand(true)
        .build();
    
    // Add glass styling to scrolled window
    scrolled_window.add_css_class("glass-scrolled");

    // Create submit button with glass styling
    let submit_button = Button::builder()
        .label("Send to Obsidian")
        .css_classes(vec!["glass-button".to_string(), "suggested-action".to_string()])
        .build();

    // Create button box for proper alignment
    let button_box = Box::new(gtk4::Orientation::Horizontal, 6);
    button_box.set_halign(gtk4::Align::End);
    button_box.add_css_class("glass-button-box");
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

