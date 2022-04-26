mod lib;

use sysinfo::{System, SystemExt};

use std::time::Duration;
use std::thread;

use lib::*;

const STRING_SEGREGATOR: &str = "====";

fn println_with_segregator(message: &str) {
	println!("{} {} {}", STRING_SEGREGATOR, message, STRING_SEGREGATOR);
}

fn main() {
	let one_second = Duration::from_secs(1);

	let mut sys = System::new_all();
	sys.refresh_all();

	println!("Loading Overview...\n");
	thread::sleep(one_second);

	sys.refresh_all();

	let cpu_info = get_cpu_info(&sys);
	let memory_info = get_memory_info(&sys);
	let disk_infos = get_disk_infos(&sys);

	println_with_segregator("System Overview");

	println!(
		"\nCPU: {}\nCPU Frequency: {:.1} GHz\nCPU Usage: {:.1}%", 
		cpu_info.name, 
		satisfy_via_magnitude(cpu_info.frequency, 3), 
		cpu_info.usage
	);

	println!(
		"\nMemory Used: {:.1} GB\nMemory Available: {:.1} GB\nTotal Memory: {:.1} GB\n",
		format_data_representation(memory_info.used, 3),
		format_data_representation(memory_info.available, 3),
		format_data_representation(memory_info.total, 3),
	);

	for disk in disk_infos {
		println!(
			"Disk Name: {}\nDisk Type: {}\nSpace Used: {:.1} GB\nSpace Available: {:.1} GB\nTotal Space: {:.1} GB\n", 
			disk.name,
			disk_variant_to_string(&disk.variant),
			format_data_representation(disk.used_space, 3),
			format_data_representation(disk.free_space, 3),
			format_data_representation(disk.total_space, 3),
		);
	}

	println_with_segregator("End Of Overview");
}
