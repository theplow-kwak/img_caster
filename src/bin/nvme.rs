use clap::{Parser, Subcommand};
use img_caster::dev::dev_utils::{NvmeController, NvmeControllerList, PhysicalDisk};
use img_caster::dev::nvme_commands::{
    print_nvme_identify_controller_data, print_nvme_identify_namespace_data,
};
use img_caster::dev::nvme_define::{NVME_CDW10_IDENTIFY, NVME_IDENTIFY_CNS_CODES};

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
}

#[derive(Subcommand)]
enum Commands {
    /// Controller List
    List {},
    /// Namespace List
    ListNs {
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
    /// Identify Controller
    IdCtrl {},
    /// Identify Namespace
    IdNs {
        /// nsid
        #[clap(short, long, default_value = "1")]
        nsid: u32,
    },
    /// Get log page
    GetLog {
        /// log id
        #[clap(short, long)]
        lid: Option<u32>,
    },
}

struct CliManager<'a> {
    args: Args,
    disk: Option<PhysicalDisk>,
    ctrl: Option<NvmeController>,
    nvme_list: &'a mut NvmeControllerList, // Add this line to store the controller list
}

impl<'a> CliManager<'a> {
    fn new(nvme_list: &'a mut NvmeControllerList) -> Self {
        let args = Args::parse();
        Self {
            args,
            disk: None,
            ctrl: None,
            nvme_list,
        }
    }
    fn open_device(&mut self) -> &mut Self {
        if let Some(driveno) = self.args.disk {
            if let Some(disk) = self.nvme_list.by_num(driveno) {
                disk.open();
                self.disk = Some(disk.clone());
            }
        }
        if let Some(busno) = self.args.bus {
            if let Some(ctrl) = self.nvme_list.by_bus(busno) {
                ctrl.open();
                self.ctrl = Some(ctrl.clone());
            }
        }
        self
    }

    fn run(&self) {
        self.disk_manager();
        self.ctrl_manager();
        self.cli_common();
    }

    fn disk_manager(&self) {
        if let Some(disk) = &self.disk {
            let device = disk.get_driver();
            match &self.args.command {
                Some(Commands::IdCtrl {}) => {
                    let info = device.nvme_identify_controller().unwrap();
                    print_nvme_identify_controller_data(&info);
                }
                Some(Commands::IdNs { nsid }) => {
                    let info = device.nvme_identify_namespace(*nsid).unwrap();
                    print_nvme_identify_namespace_data(&info);
                }
                Some(Commands::ListNs { all }) => {
                    let mut cdw10 = NVME_CDW10_IDENTIFY::default();
                    let cns = if *all {
                        NVME_IDENTIFY_CNS_CODES::NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST as u8
                    } else {
                        NVME_IDENTIFY_CNS_CODES::NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES as u8
                    };
                    cdw10.set_CNS(cns);
                    let info = device.nvme_identify_query(cdw10.into(), 0).unwrap();
                    println!("{:?}", &info[..20 as usize]);
                }
                Some(Commands::GetLog { lid }) => {
                    let info = device.nvme_logpage_query(lid.unwrap_or(2), 0).unwrap();
                    println!("{:?}", info);
                }
                _ => {}
            }
        };
    }

    fn ctrl_manager(&self) {
        if let Some(controller) = &self.ctrl {
            let device = controller.get_driver();
            match &self.args.command {
                Some(Commands::ListNs { all }) => {
                    device.nvme_identify_ns_list(0, *all).unwrap();
                }
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
                _ => {}
            }
        }
    }

    fn cli_common(&self) {
        match &self.args.command {
            Some(Commands::List {}) => {
                if let Some(ctrl) = &self.ctrl {
                    println!("NVME: {}", ctrl);
                } else {
                    println!("{}", self.nvme_list);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut controller_list = NvmeControllerList::new();
    controller_list.enumerate();

    let mut cli = CliManager::new(&mut controller_list);
    cli.open_device();
    cli.run();
}
