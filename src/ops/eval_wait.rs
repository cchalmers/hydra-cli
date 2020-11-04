// use crate::hydra::client::{Eval, HydraClient};
use crate::hydra::client::HydraClient;
use crate::ops::{ok_msg, OpResult};
use std::collections::{BTreeMap, BTreeSet};
use std::time;

pub const RED: &str = "\u{1b}[49;31m";
pub const GREEN: &str = "\u{1b}[49;32m";
pub const YELLOW: &str = "\u{1b}[49;33m";
pub const CLEAR: &str = "\u{1b}[0m";

pub fn run(client: &dyn HydraClient, eval_id: u64) -> OpResult {
    let start_waiting_for_jobs = time::Instant::now();
    let eval = client.eval(eval_id)?;
    let mut unfinished_builds: BTreeSet<u64> = eval.builds.iter().cloned().collect();
    let mut finished_builds = BTreeMap::default();
    let mut first = true;
    let mut num_errors = 0;
    loop {
        for build_id in unfinished_builds.clone() {
            let build = match client.build(build_id * 10) {
                Ok(build) => build,
                Err(err) => {
                    num_errors += 1;
                    eprintln!(
                        "{}got a error fetching info for build {} ({} errors so far) {:?}{}",
                        RED, build_id, num_errors, err, CLEAR
                    );
                    if num_errors > 20 {
                        panic!("too many errors!");
                    } else if first {
                        panic!("failed on first pass");
                    }
                    break;
                }
            };
            if first && build.finished != 1 {
                eprintln!(
                    "{}waiting for build {} {}/build/{}{}",
                    YELLOW,
                    build.nixname,
                    client.host(),
                    build_id,
                    CLEAR
                );
            }
            if build.finished == 1 {
                let emoji = if build.buildstatus == Some(0) {
                    "✅"
                } else {
                    "❌"
                };
                println!(
                    "{} {}/build/{} {}",
                    emoji,
                    client.host(),
                    build_id,
                    build.job
                );
                unfinished_builds.remove(&build_id);
                finished_builds.insert(build_id, build);
            }
        }
        first = false;
        if unfinished_builds.is_empty() {
            break;
        };
        std::thread::sleep(time::Duration::from_secs(8));
        if start_waiting_for_jobs.elapsed() > time::Duration::from_secs(60 * 60 * 24) {
            panic!("timeout")
        }
    }

    ok_msg("jobset_eval")
}
