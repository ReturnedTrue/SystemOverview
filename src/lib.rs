use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, DiskType};

pub struct CpuInfo {
	pub name: String,
	pub frequency: u64, // MHz
	pub usage: f32,
}

pub struct MemoryInfo {
	pub used: u64, // B
	pub available: u64,
	pub total: u64,
}

pub struct DiskInfo {
	pub name: String,
	pub variant: DiskType,
	pub used_space: u64, // B
	pub free_space: u64, 
	pub total_space: u64,
}

pub fn get_cpu_info(sys: &System) -> CpuInfo {
	let cpu = sys.global_processor_info();

	let processors = sys.processors();
	let processor_count = processors.len() as u64;

	let mut calculated_frequency = 0;

	for processor_core in processors.iter() {
		calculated_frequency += processor_core.frequency();
	}

	calculated_frequency /= processor_count;

	return CpuInfo {
		name: cpu.brand().to_string(),
		frequency: calculated_frequency,
		usage: cpu.cpu_usage()
	};
}

pub fn get_memory_info(sys: &System) -> MemoryInfo {
	return MemoryInfo {
		used: sys.used_memory() * 1000,
		available: sys.free_memory() * 1000,
		total: sys.total_memory() * 1000,
	};
}

pub fn get_disk_infos(sys: &System) -> Vec<DiskInfo> {
	let system_disks = sys.disks();
	let amount_of_disks = system_disks.len();

	let mut disk_infos = Vec::<DiskInfo>::with_capacity(amount_of_disks);

	for disk in system_disks.iter() {
		let free = disk.available_space();
		let total = disk.total_space();

		let info = DiskInfo {
			name: disk.name().to_string_lossy().to_string(),
			variant: disk.type_(),
			used_space: total - free,
			free_space: free,
			total_space: total,
		};

		disk_infos.push(info);
	}

	return disk_infos;
}

pub fn disk_variant_to_string(variant: &DiskType) -> &str {
	return match variant {
		DiskType::SSD => "Solid State",
		DiskType::HDD => "Hard Disk",
		DiskType::Unknown(_num) => "Unrecognised",
	};
}

pub fn satisfy_via_magnitude(unsigned: u64, magnitude: u32) -> f32 {
	let adjusted_tenth = (10 as u64).pow(magnitude) as f32;

	return (unsigned as f32) / adjusted_tenth;
}

pub fn format_data_representation(bytes_amount: u64, order: u32) -> f32 {
	let amount_in_representation = (1024 as u32).pow(order) as f32;

	return bytes_amount as f32 / amount_in_representation;
}