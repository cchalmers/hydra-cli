use crate::hydra::client::{Eval, HydraClient};
use crate::ops::{eval_wait, ok_msg, OpResult};
use std::collections::BTreeSet;
use std::time;

pub fn run(
    client: &dyn HydraClient,
    project_name: &str,
    jobset_name: &str,
    get_eval_id: bool,
    wait_eval: bool,
) -> OpResult {
    if !get_eval_id || !wait_eval {
        client.jobset_eval(project_name, jobset_name)?;
        return ok_msg("jobset_eval");
    }

    let evals_before = client.jobset_evals(project_name, jobset_name)?;
    client.jobset_eval(project_name, jobset_name)?;

    let eval_ids: BTreeSet<u64> = evals_before.evals.iter().map(|eval| eval.id).collect();

    let start_waiting_for_eval = time::Instant::now();
    let mut new_evals: Vec<Eval>;
    loop {
        new_evals = client
            .jobset_evals(project_name, jobset_name)?
            .evals
            .into_iter()
            .filter(|eval| !eval_ids.contains(&eval.id))
            .collect();
        // TODO: detect if this is a duplicate evaluation (in which case it won't come up as new)
        if !new_evals.is_empty() {
            break;
        };
        std::thread::sleep(time::Duration::from_secs(1));
        if start_waiting_for_eval.elapsed() > time::Duration::from_secs(60 * 10) {
            panic!("timeout")
        }
    }

    if new_evals.len() > 1 {
        eprintln!("warning! more than more evaluation detected, going to wait for the first")
    }
    let new_eval = &new_evals[0];

    println!(
        "evaluation started: https://hydra.myrtle/eval/{}",
        new_eval.id
    );

    if wait_eval {
        eval_wait::run(client, new_eval.id)?;
    }

    ok_msg("jobset_eval")
}
