use crate::hydra::client::{Build, Eval};
use chrono::NaiveDateTime;

pub fn evaluation_pretty_print(e: &Eval) {
    for (k, v) in &e.jobsetevalinputs {
        println!("  {}", k);
        println!("    {:10} {}", "type", v.input_type);
        if let Some(t) = &v.value {
            println!("    {:10} {}", "value", t);
        }
        if let Some(t) = &v.uri {
            println!("    {:10} {}", "uri", t);
        }
        if let Some(t) = &v.revision {
            println!("    {:10} {}", "revision", t);
        }
    }
}

pub fn build_pretty_print(b: &Build) {
    println!("{:14} {}/{}/{}", "Job", b.project, b.jobset, b.job);
    if let Some(stoptime) = b.stoptime {
        println!(
            "{:14} {}",
            "Finished at",
            NaiveDateTime::from_timestamp(stoptime, 0),
        );
    }
    println!("{:14} {}", "Derviation", b.drvpath);
    println!("{:14}", "Build outputs");
    for (k, v) in &b.buildoutputs {
        println!("  {:12} {}", k, v.path);
    }
}
