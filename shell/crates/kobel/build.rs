use vergen_git2::{BuildBuilder, CargoBuilder, Emitter, RustcBuilder, SysinfoBuilder, Git2Builder};

pub fn main() {
    let build = BuildBuilder::all_build().expect("Failed to create BuildBuilder");
    let cargo = CargoBuilder::all_cargo().expect("Failed to create CargoBuilder");
    let git2 = Git2Builder::all_git().expect("Failed to create Git2Builder");
    let rustc = RustcBuilder::all_rustc().expect("Failed to create RustcBuilder");
    let si = SysinfoBuilder::all_sysinfo().expect("Failed to create SysinfoBuilder");

    Emitter::default()
        .add_instructions(&build)
        .expect("Failed to add build instructions")
        .add_instructions(&cargo)
        .expect("Failed to add cargo instructions")
        .add_instructions(&rustc)
        .expect("Failed to add rustc instructions")
        .add_instructions(&git2)
        .expect("Failed to add git2 instructions")
        .add_instructions(&si)
        .expect("Failed to add sysinfo instructions")
        .emit()
        .expect("Failed to emit vergen data");
}