use std::{fs, mem::MaybeUninit, os::linux::fs::MetadataExt, thread, time};

use libbpf_rs::{
    Result,
    skel::{OpenSkel, Skel, SkelBuilder},
};

use crate::minimal_ns::MinimalNsSkelBuilder;

mod minimal_ns {
    include!(concat!(env!("OUT_DIR"), "/minimal_ns.skel.rs"));
}

fn main() -> Result<()> {
    let skel_builder = MinimalNsSkelBuilder::default();
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
    let meta = fs::metadata("/proc/self/ns/pid")?;
    bss.dev = meta.st_dev();
    bss.ino = meta.st_ino();

    skel.attach()?;

    println!(
        "Successfully started! Please run `sudo cat /sys/kernel/debug/tracing/trace_pipe` to see output of the BPF programs."
    );

    loop {
        eprint!(".");
        thread::sleep(time::Duration::from_secs(1));
    }
}
