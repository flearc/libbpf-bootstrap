use std::{mem::MaybeUninit, thread, time};

use libbpf_rs::{
    ErrorExt, MapCore, MapFlags, Result,
    skel::{OpenSkel, Skel, SkelBuilder},
};

use crate::minimal_legacy::MinimalLegacySkelBuilder;

mod minimal_legacy {
    include!(concat!(env!("OUT_DIR"), "/minimal_legacy.skel.rs"));
}

fn main() -> Result<()> {
    let skel_builder = MinimalLegacySkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = skel_builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;

    let index: i32 = 0;
    let pid: i32 = std::process::id() as i32;
    skel.maps
        .my_pid_map
        .update(&index.to_le_bytes(), &pid.to_le_bytes(), MapFlags::ANY)
        .context("update my_pid_map fail")?;

    skel.attach()?;

    println!(
        "Successfully started! Please run `sudo cat /sys/kernel/debug/tracing/trace_pipe` to see output of the BPF programs."
    );

    loop {
        eprint!(".");
        thread::sleep(time::Duration::from_secs(1));
    }
}
