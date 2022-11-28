use clap::Parser;
use kll_compiler::{Filestore, KllDatastore, KllGroups};
use layouts_rs::Layouts;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, enum_utils::FromStr)]
#[enumeration(rename_all = "lowercase")]
pub enum EmitterType {
    /// Re-generate kll files
    Kll,
    /// Old C-style code generation
    Kiibohd,
    /// Configurator data format
    Configurator,
    /// Simple rust code generation (for basic unit tests)
    Rust,
    /// Not set
    None,
}

#[allow(dead_code)]
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct CliOpts {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[clap(short, long)]
    #[structopt(short, long)]
    debug: bool,

    /// Specify target emitter for the KLL compiler. Pass multiple times to use more than one.
    #[clap(long, value_parser, value_name = "kiibohd")]
    emitter: String,

    /// Specify base configuration .kll files, earliest priority
    /// Contains capabilities, defines, and other similar information
    #[clap(long, value_parser)]
    config: Vec<PathBuf>,

    /// Specify base map configuration, applied after config .kll files.
    /// The base map is applied prior to all default and partial maps and is used as the basis for them.
    #[clap(long, value_parser)]
    base: Vec<PathBuf>,

    /// Specify .kll files to layer on top of the default map to create a combined map.
    /// Also known as layer 0.
    #[clap(long, value_parser)]
    default: Vec<PathBuf>,

    /// Specify .kll files to generate partial map, multiple files per flag.
    /// Each -p defines another partial map (new layer)
    #[clap(short, long, value_parser)]
    partial: Vec<PathBuf>,

    #[structopt(flatten)]
    kiibohd: KiibohdOpts,
}

#[allow(dead_code)]
#[derive(Debug, Parser)]
struct KiibohdOpts {
    /// Specify KLL define .h file output.
    #[clap(long, value_parser, value_name = "kll_defs.h")]
    def_output: PathBuf,

    /// Specify USB HID Lookup .h file output.
    #[clap(long, value_parser, value_name = "usb_id.h")]
    hid_output: PathBuf,

    /// Specify KLL map .h file output (key bindings)
    #[clap(long, value_parser, value_name = "generatedKeymap.h")]
    map_output: PathBuf,

    /// Specify KLL map .h file output. (animation and lighting)
    #[clap(long, value_parser, value_name = "generatedPixelmap.h")]
    pixel_output: PathBuf,

    /// Specify json output file for settings dictionary.
    #[clap(long, value_parser, value_name = "kll.json")]
    json_output: PathBuf,
}

fn main() {
    let args = CliOpts::parse();
    if args.debug {
        println!("=== ARGS === \n{:#?}", &args);
    }

    let mut filestore = Filestore::new();
    for file in args
        .config
        .iter()
        .chain(&args.base)
        .chain(&args.default)
        .chain(&args.partial)
    {
        filestore.load_file(file);
    }

    let groups = KllGroups::new(
        &filestore,
        &args.config,
        &args.base,
        &args.default,
        &args.partial,
    );
    if args.debug {
        println!("=== CONFIG  === \n{:#?}", groups.config());
        println!("=== DEFAULT === \n{:#?}", groups.defaultmap());
        println!("=== PARTIAL === \n{:#?}", groups.partialmaps());
    }

    let emitter = EmitterType::from_str(&args.emitter).unwrap();
    // Old C-based kiibohd firmware
    if emitter == EmitterType::Kiibohd {
        let mut layouts = Layouts::from_dir(PathBuf::from("layouts"));
        let layout = layouts.get_layout("base/base.json");
        dbg!(&layout.from_hid_keyboard);

        let mut defaultmap = groups.defaultmap();
        defaultmap.keymap = defaultmap.reduce(groups.basemap());
        for s in &defaultmap.keymap {
            println!("{}", s);
        }

        if args.debug {
            let kll_data = KllDatastore::new(&defaultmap);
            println!("{:?}", kll_data);
        }

        let outfile = env::current_dir().unwrap().join("generatedKeymap.h");
        kll_compiler::emitters::kiibohd::write(&outfile, &groups);
        println!("Wrote {:?}", outfile);
    }
}
