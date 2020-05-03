mod mem_lib;

fn main() {
    let pid = mem_lib::get_proc_id_by_name("dota2.exe");
    println!("Found process with id {}", pid);
    // mem_lib::get_proc_id_by_name("dota2.exe");

    let process = mem_lib::get_proc(pid);

    println!("Read {}", process.read_address("0x0"));
}
