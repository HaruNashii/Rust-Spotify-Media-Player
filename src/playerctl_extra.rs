use std::process::Command;

pub fn command(command: &str) -> String {
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();

    let stdout = Command::new(parts.remove(0))
        .args(parts)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute command '{}'", command))
        .stdout;

    String::from_utf8(stdout).expect("Stdout was not valid UTF-8")
}


// VOLUME
pub fn volume_get() -> String {
    let mut volume = command("playerctl volume");

    while volume.len() >= 4 { volume.pop(); }

    volume
}

pub fn volume_up(amount: f32) {
    command(&format!("playerctl volume {}+", amount));
}

pub fn volume_down(amount: f32) {
    command(&format!("playerctl volume {}-", amount));
}

// TIME
pub fn total_time_get() -> String {
    command("playerctl --player=spotify -s metadata --format '{{duration(mpris:length)}}'")
}

pub fn current_time_get() -> String {
    command("playerctl --player=spotify -s position --format '{{duration(position)}}'")
}

pub fn remaining_time_get() -> String {
    command("playerctl --player=spotify -s metadata --format '{{duration(mpris:length-position)}}'")
}

// this is only necessary because the crate "Playerctl" Has one ERROR in the "ShuffleStatus" enum
// which the options of the enum is not set to public, so it can't be used.
pub fn shuffle_toggle_set() {
    command("playerctl --player=spotify -s shuffle Toggle");
}

pub fn shuffle_get() -> String {
    command("playerctl --player=spotify shuffle")
}

pub fn status_get() -> String {
    command("playerctl --player=spotify status")
}

// MISC
pub fn art_url_get() -> String {
    command("playerctl --player=spotify -s metadata mpris:artUrl")
}
