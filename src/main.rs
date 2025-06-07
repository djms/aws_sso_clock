use anyhow::{Ok, Result};
use chrono::{Duration, TimeZone, Utc};
use std::fs;

/*
.p10k.zsh

# Custom
function prompt_aws_sso_time_left(){
  echo "working!!!"
  local sso_info
  sso_info=$(~/aws-sso-clock 2> /dev/null)
  if [[ -n $sso_info ]];
  then
  p10k segment -b yellow -f black -t "$sso_info"
  fi
}

typeset -g POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(
aws_sso_time_left
)
*/

fn main() -> Result<()> {
    let ts_path = "/Users/peter/.aws/sso-login-start".to_string();
    let ts = fs::read_to_string(ts_path)?.trim().parse::<i64>()?;
    let issued_at = Utc.timestamp_opt(ts, 0).single().unwrap();
    let sso_duration = Duration::hours(6);
    let sso_expires = issued_at + sso_duration;
    let now = Utc::now();
    let remaining = sso_expires - now;
    let min = remaining.num_minutes();
    let hours = min / 60;
    let min = min % 60;
    print!("{hours:02}:{min:02}");
    Ok(())
}
