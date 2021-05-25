use std::convert::TryInto;

use aya::{programs::SchedClassifier, Bpf};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {:#}", e);
    }
}

fn try_main() -> Result<(), anyhow::Error> {
    let code = include_bytes!("../aya-template-bpf/target/bpfel-unknown-none/debug/aya-template-bpf").to_vec();
    let mut bpf = Bpf::load(&code, None)?;
    let prog: &mut SchedClassifier = bpf.program_mut("egress")?.try_into()?;
    prog.load()?;
    //prog.attach(...)?;

    Ok(())
}
