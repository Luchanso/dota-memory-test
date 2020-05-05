mod mem_lib;

fn main() {
    let process_name = "dota2.exe";
    let pid = mem_lib::get_proc_id_by_name(process_name);
    println!("Found {} with id {}", process_name, pid);

    let process = mem_lib::get_proc(pid);

    println!("Read {}", process.read_address("0x0"));
}
