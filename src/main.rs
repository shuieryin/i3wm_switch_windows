use std::{env, io, process};

use tokio_i3ipc::I3;

use crate::state::{collect_window_ids, SwitchMode, STATE};

mod state;

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

    collect_window_ids(&tree);

    let focused_window_id = STATE.get_focused_window_id();
    if STATE.get_first_window_id() == focused_window_id
        && STATE.get_last_window_id() == focused_window_id
    {
        println!("i3wm_switch_windows: no prev or next window is focusable");
    } else {
        let target_window_id = match mode {
            SwitchMode::Forward => {
                let next_window_id = STATE.get_next_window_id();
                if next_window_id > 0 {
                    next_window_id
                } else {
                    STATE.get_first_window_id()
                }
            }
            SwitchMode::Backward => {
                let prev_window_id = STATE.get_prev_window_id();
                if prev_window_id > 0 {
                    prev_window_id
                } else {
                    STATE.get_last_window_id()
                }
            }
        };

        let results = i3
            .run_command(format!("[con_id=\"{target_window_id}\"] focus"))
            .await?;
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
