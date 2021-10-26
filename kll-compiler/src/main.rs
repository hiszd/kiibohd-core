use kll::{Filestore, KllDatastore, KllGroups};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq, enum_utils::FromStr)]
#[enumeration(rename_all = "lowercase")]
pub enum EmitterType {
    Kll,
    Kiibohd,
    Configurator,
    Rust,
    None,
}

#[derive(Debug, StructOpt)]
struct CliOpts {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Specify target emitter for the KLL compiler. Pass multiple times to use more than one.
    #[structopt(long, default_value = "kiibohd")]
    emitter: String,

    /// Specify base configuration .kll files, earliest priority
    /// Contains capabilities, defines, and other similar information
    #[structopt(long, parse(from_os_str))]
    config: Vec<PathBuf>,

    /// Specify base map configuration, applied after config .kll files.
    /// The base map is applied prior to all default and partial maps and is used as the basis for them.
    #[structopt(long, parse(from_os_str))]
    base: Vec<PathBuf>,

    /// Specify .kll files to layer on top of the default map to create a combined map.
    /// Also known as layer 0.
    #[structopt(long, parse(from_os_str))]
    default: Vec<PathBuf>,

    /// Specify .kll files to generate partial map, multiple files per flag.
    /// Each -p defines another partial map (new layer)
    #[structopt(long, parse(from_os_str))]
    partial: Vec<PathBuf>,

    #[structopt(flatten)]
    kiibohd: KiibohdOpts,
}

#[derive(Debug, StructOpt)]
struct KiibohdOpts {
    /// Specify KLL define .h file output.
    #[structopt(long, parse(from_os_str), default_value = "kll_defs.h")]
    def_output: PathBuf,

    /// Specify USB HID Lookup .h file output.
    #[structopt(long, parse(from_os_str), default_value = "usb_id.h")]
    hid_output: PathBuf,

    /// Specify KLL map .h file output (key bindings)
    #[structopt(long, parse(from_os_str), default_value = "generatedKeymap.h")]
    map_output: PathBuf,

    /// Specify KLL map .h file output. (animation and lighting)
    #[structopt(long, parse(from_os_str), default_value = "generatedPixelmap.h")]
    pixel_output: PathBuf,

    /// Specify json output file for settings dictionary.
    #[structopt(long, parse(from_os_str), default_value = "kll.json")]
    json_output: PathBuf,
}

fn main() {
    let args = CliOpts::from_args();
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
    match emitter {
        EmitterType::Kiibohd => {
            if args.debug {
                let defaultmap = groups.defaultmap();
                let kll_data = KllDatastore::new(&defaultmap);
                println!("{:?}", kll_data);
            }

            let outfile = env::current_dir().unwrap().join("generatedKeymap.h");
            kll::emitters::kiibohd::write(&outfile, &groups);
            println!("Wrote {:?}", outfile);
        }
        _ => {}
    }
}
