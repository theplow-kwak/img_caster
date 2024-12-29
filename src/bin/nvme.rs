use clap::{Parser, Subcommand};
use img_caster::dev::dev_utils::NvmeControllerList;
use img_caster::dev::nvme_commands::{nvme_identify_query, print_nvme_identify_controller_data};
use img_caster::dev::nvme_define::{
    NVME_ADMIN_COMMANDS, NVME_COMMAND, NVME_IDENTIFY_CONTROLLER_DATA,
};
use img_caster::dev::nvme_device::NvmeDevice;

#[derive(Parser, Default)]
#[command(author, version, about)]
/// Sender for Multicast File Transfer
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// PhysicalDrive number. ex) 1 -> "\\.\PhysicalDrive1"
    #[arg(short, long)]
    disk: Option<i32>,

    /// pci bus number. ex) 3 -> "3:0.0"
    #[arg(short, long)]
    bus: Option<i32>,

    /// Namespace ID
    #[arg(short, long)]
    nsid: Option<i32>,
}

#[derive(Subcommand)]
enum Commands {
    /// Controller List
    List {},
    /// Namespace List
    Listns {
        #[arg(short, long)]
        all: bool,
    },
    /// Creates a namespace
    Create {
        /// size of ns (NSZE)
        #[clap(short, long)]
        size: Option<i32>,
    },
    /// Deletes a namespace from the controller
    Delete {},
    /// Attaches a namespace to requested controller(s)
    Attach {},
    /// Detaches a namespace from requested controller(s)
    Detach {},
}

fn main() {
    let args = Args::parse();

    let mut controller_list = NvmeControllerList::new();
    controller_list.enumerate();

    let mut controller = None;
    if let Some(driveno) = args.disk {
        controller = controller_list.by_num(driveno);
    }
    if let Some(busno) = args.bus {
        controller = controller_list.by_bus(busno);
    }

    match controller {
        Some(controller) => {
            let device = NvmeDevice::open(&controller.path()).unwrap();
            let info = nvme_identify_query(&device).unwrap();
            print_nvme_identify_controller_data(&info);

            match &args.command {
                Some(Commands::List {}) => {
                    println!("{}", controller);
                }
                Some(Commands::Listns { all }) => {}
                Some(Commands::Create { size }) => {
                    controller.rescan();
                }
                Some(Commands::Delete {}) => {
                    controller.remove();
                }
                Some(Commands::Attach {}) => {
                    controller.enable();
                }
                Some(Commands::Detach {}) => {
                    controller.disable();
                }
                None => {}
            }
        }
        None => {
            println!("{}", controller_list);
        }
    };
}
