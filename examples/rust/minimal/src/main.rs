use std::{mem::MaybeUninit, thread, time};

use libbpf_rs::{
    Result,
    skel::{OpenSkel, Skel, SkelBuilder},
};

use crate::minimal::MinimalSkelBuilder;

mod minimal {
    include!(concat!(env!("OUT_DIR"), "/minimal.skel.rs"));
}

fn main() -> Result<()> {
    let skel_builder = MinimalSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = skel_builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;
    let bss = skel
        .maps
        .bss_data
        .as_deref_mut()
        .expect("`bss` is not memory mapped");

    let pid = std::process::id() as i32;
    bss.my_pid = pid;

    skel.attach()?;

    println!(
        "Successfully started! Please run `sudo cat /sys/kernel/debug/tracing/trace_pipe` to see output of the BPF programs."
    );
    loop {
        eprint!(".");
        thread::sleep(time::Duration::from_secs(1));
    }
}
