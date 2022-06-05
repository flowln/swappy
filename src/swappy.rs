use die::die;
use gio::{resources_register, ApplicationCommandLine, ApplicationFlags, Resource};
use glib::{Bytes, Char, OptionArg, OptionFlags, Value};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, DrawingArea};
use version::version;

use gtk::gio;
use gtk::glib;

use crate::config::Config;

pub struct State {
    config: Config,
}

impl State {
    pub fn new() -> State {
        State {
            config: Config::new("test"),
        }
    }
}

fn build_layout(_: &Application) {
    let res_bytes = include_bytes!("../res/swappy.gresource");
    let data = Bytes::from(&res_bytes[..]);
    let resource = Resource::from_data(&data).unwrap();
    resources_register(&resource);

    let builder = Builder::from_resource("/me/jtheoof/swappy/swappy.glade");

    let window: ApplicationWindow = builder
        .object("paint-window")
        .expect("could not find paint-window in glade file");

    let paint_area: DrawingArea = builder
        .object("painting-area")
        .expect("could not find painting-area in glade file");

    paint_area.connect_resize(move |_, w, h| {
        println!("paint-area resized {}x{}", w, h);
    });

    window.show();
}

fn on_handle_local_options(_app: &Application, options: &glib::VariantDict) -> i32 {
    if options.contains("version") {
        println!("swappy version {}", version!());
        return 0;
    }
    let maybe_file = options.lookup_value("file", None);
    match maybe_file {
        None => die!("no geometry found, did you use -f option?"),
        Some(file) => {
            println!("file is {}", file);
        }
    }
    -1
}

fn on_command_line_connected(app: &Application, _: &ApplicationCommandLine) -> i32 {
    // build_ui(app);
    println!("'command-line' called");
    build_layout(app);
    0
}

pub fn init() {
    let app = Application::new(
        Some("me.jtheoof.swappy"),
        ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    app.add_main_option(
        "version",
        Char::from(b'v'),
        OptionFlags::NONE,
        OptionArg::None,
        "Print version and quit",
        None,
    );

    app.add_main_option(
        "file",
        Char::from(b'f'),
        OptionFlags::NONE,
        OptionArg::String,
        "Load a file at a specific path",
        None,
    );

    app.add_main_option(
        "output-file",
        Char::from(b'o'),
        OptionFlags::NONE,
        OptionArg::String,
        "Print the final surface to the given file when exiting, use - to print to stdout",
        None,
    );

    app.connect_handle_local_options(on_handle_local_options);
    app.connect_command_line(on_command_line_connected);

    app.run();
}
