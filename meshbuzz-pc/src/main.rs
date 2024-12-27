struct VirtualMeshBuzz;

impl meshbuzz_base::Hardware for VirtualMeshBuzz {
    fn log_message(message: &str) {
        println!("{}", message);
    }
}

fn main() {
    let timelet = timelets::Timelet::<VirtualMeshBuzz>::new();
    timelet.run();
}
