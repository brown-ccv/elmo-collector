use crate::{CpuUtilization, GpuUtilization, NodeType, NodeUtilization};
use std::process::Command;

pub fn get_cpu_info() -> CpuUtilization {
    let output = Command::new("bash")
        .arg("-c")
        .arg("sinfo --all --format=%C --noheader")
        .output()
        .expect("Failed to execute command to get cpu info");

    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let nums = output_str
        .split('/')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let n_alloc = nums[0];
    let n_idle = nums[1];
    let n_other = nums[2];
    let n_total = nums[3];

    CpuUtilization {
        n_alloc,
        n_idle,
        n_other,
        n_total,
    }
}
pub fn get_gpu_info() -> GpuUtilization {
    let n_alloc = Command::new("bash")
        .arg("-c")
        .arg("squeue -h -t R -O gres,nodelist | rg gpu: | cut -d: -f 2 | cut -d ' ' -f 1 | grep -o '[0-9]\\+' | paste -sd+ - | b")
        .output()
        .expect("Failed to execute command to get allocated GPUs");

    let n_total = Command::new("bash")
        .arg("-c")
        .arg("sinfo -o \"%n %G\" | rg gpu: | cut -d':' -f 3 | sed 's/[^0-9]*//g' | paste -sd+ | bc")
        .output()
        .expect("Failed to execute command to get total GPU count");

    GpuUtilization {
        n_alloc: String::from_utf8_lossy(&n_alloc.stdout)
            .trim()
            .parse::<i32>()
            .unwrap(),
        n_total: String::from_utf8_lossy(&n_total.stdout)
            .trim()
            .parse::<i32>()
            .unwrap(),
    }
}

pub fn get_powersave_node_info(node_type: NodeType) -> NodeUtilization {
    let numerator_cmd = match node_type {
        NodeType::All => "scontrol show nodes | grep State=IDLE+POWERED_DOWN | wc -l",
        NodeType::Cpu => {
            "scontrol show nodes | awk '/NodeName=node[0-9]+/{node=$0} /State=IDLE\\+POWERED_DOWN/{if (node) print node; node=\"\"}' | wc -l"
        }
        NodeType::Gpu => {
            "scontrol show nodes | awk '/NodeName=gpu[0-9]+/{node=$0} /State=IDLE\\+POWERED_DOWN/{if (node) print node; node=\"\"}' | wc -l"
        }
    };

    let denominator_cmd = match node_type {
        NodeType::All => "sinfo --Node |cut -d \" \" -f 1|tail -n +2 | sort| uniq | wc -l",
        NodeType::Cpu => {
            "scontrol show nodes | awk '/NodeName=node[0-9]+/{node=$0} {if (node) print node; node=\"\"}' | wc -l"
        }
        NodeType::Gpu => {
            "scontrol show nodes | awk '/NodeName=gpu[0-9]+/{node=$0} {if (node) print node; node=\"\"}' | wc -l"
        }
    };

    let n_alloc = Command::new("bash")
        .arg("-c")
        .arg(numerator_cmd)
        .output()
        .expect("Failed to execute command to get nodes in powersave mode");

    let n_total = Command::new("bash")
        .arg("-c")
        .arg(denominator_cmd)
        .output()
        .expect("Failed to execute command to get total count of nodes");

    NodeUtilization {
        n_alloc: String::from_utf8_lossy(&n_alloc.stdout)
            .trim()
            .parse::<i32>()
            .unwrap(),
        n_total: String::from_utf8_lossy(&n_total.stdout)
            .trim()
            .parse::<i32>()
            .unwrap(),
    }
}
