use config::Config;
use die::die;
use gio::prelude::*;
use gio::{resources_register, ApplicationCommandLine, ApplicationExt, ApplicationFlags, Resource};
use glib::{Bytes, Char, OptionArg, OptionFlags, Value};
use gtk::prelude::*;
use gtk::{Application, Builder};
use std::env::args;
use version::version;

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

fn draw_area_handler(_val: &[Value]) -> Option<Value> {
    println!("'draw' event called");
    for v in _val {
        println!("{:?}", v);
    }

    Some(false.to_value())
}

fn build_layout(_: &Application) {
    let res_bytes = include_bytes!("../res/swappy.gresource");
    let data = Bytes::from(&res_bytes[..]);
    let resource = Resource::from_data(&data).unwrap();
    resources_register(&resource);

    let builder = Builder::from_resource("/me/jtheoof/swappy/swappy.glade");

    builder.connect_signals(move |_, handler_name| {
        println!("handler name {}", handler_name);
        if handler_name == "draw_area_handler" {
            Box::new(draw_area_handler)
        } else {
            Box::new(|_| None)
        }
    });
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
    )
    .expect("Initialization failed...");

    app.add_main_option(
        "version",
        Char::new('v').unwrap(),
        OptionFlags::NONE,
        OptionArg::None,
        "Print version and quit",
        None,
    );

    app.add_main_option(
        "file",
        Char::new('f').unwrap(),
        OptionFlags::NONE,
        OptionArg::String,
        "Load a file at a specific path",
        None,
    );

    app.add_main_option(
        "output-file",
        Char::new('o').unwrap(),
        OptionFlags::NONE,
        OptionArg::String,
        "Print the final surface to the given file when exiting, use - to print to stdout",
        None,
    );

    app.connect_handle_local_options(on_handle_local_options);
    app.connect_command_line(on_command_line_connected);

    app.run(&args().collect::<Vec<_>>());

    gtk::main();
}
