use std::{env, io, process};
use tokio_i3ipc::I3;
use tokio_i3ipc::reply::NodeType::Con;

#[derive(Debug)]
enum SwitchMode {
    Forward,
    Backward
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut mode = SwitchMode::Forward;
    if let Some(arg) = args.get(1) {
        if arg == "backward" {
            mode = SwitchMode::Backward;
        }
    }

    let mut i3 = I3::connect().await?;

    let tree = i3.get_tree().await?;
    let mut focused_window_id = 0;
    let mut prev_window_id = 0;
    let mut next_window_id = 0;
    let mut first_window_id = 0;
    let mut last_window_id = 0;
    for output in tree.nodes {
        if let Some(output_name) = output.name {
            if output_name != "__i3" {
                for container in output.nodes {
                    if container.node_type == Con {
                        for workspace in container.nodes {
                            for window in workspace.nodes {
                                if first_window_id == 0 {
                                    first_window_id = window.id;
                                }

                                if window.focused {
                                    focused_window_id = window.id;
                                } else {
                                    if focused_window_id == 0 {
                                        prev_window_id = window.id;
                                    } else if next_window_id == 0 {
                                        next_window_id = window.id;
                                    }
                                }

                                last_window_id = window.id;
                            }
                        }
                    }
                }
            }
        }
    }

    if first_window_id == focused_window_id && last_window_id == focused_window_id {
        println!("i3wm_switch_windows: no prev or next window is focusable");
    } else {
        let target_window_id = match mode {
            SwitchMode::Forward => {
                if next_window_id > 0 {
                    next_window_id
                } else {
                    first_window_id
                }
            }
            SwitchMode::Backward => {
                if prev_window_id > 0 {
                    prev_window_id
                } else {
                    last_window_id
                }
            }
        };

        let results = i3.run_command(format!("[con_id=\"{target_window_id}\"] focus")).await?;
        let mut is_success = true;
        for result in results {
            if let Some(error) = result.error {
                eprintln!("i3wm_switch_windows: {error}");
                is_success = false;
            }
        }
        if is_success {
            println!("i3wm_switch_windows: switched to {target_window_id}");
        } else {
            process::exit(1);
        }
    }

    Ok(())
}