use crate::func;

pub fn help() {
   func::msg_ok("USAGE: kargo <FUNC>");
   func::msg_info("  h/help     - show this message");
   func::msg_info("                 ↳ kargo help");
   func::msg_info("  i/info     - about kargo");
   func::msg_info("                 ↳ kargo info");
   func::msg_info("  mk/make    - make a new cargo crate");
   func::msg_info("                 ↳ kargo mk (blank = dir is crate name)");
   func::msg_info("  n/new          ↳ kargo mk <crate_name>");
   func::msg_info("                 ↳ kargo mk --lib");
   func::msg_info("  wrk/ws     - scaffold a minimal bin+lib workspace");
   func::msg_info("                 ↳ kargo wrk <lib> <bin>");
   func::msg_info("  r/rld      - rebuild & install current crate");
   func::msg_info("                 ↳ kargo r");
}

pub fn info() {
   func::msg_ok("INFO: kargo");
   func::msg_info(&format!("  name      : {}", env!("CARGO_PKG_NAME")));
   func::msg_info(&format!("  desc      : {}", env!("CARGO_PKG_DESCRIPTION")));
   func::msg_info(&format!("  version   : {}", env!("CARGO_PKG_VERSION")));
   func::msg_info(&format!("  author    : {}", env!("CARGO_PKG_AUTHORS")));
   func::msg_info(&format!("  license   : {}", env!("CARGO_PKG_LICENSE")));
   func::msg_info(&format!("  repo      : {}", env!("CARGO_PKG_REPOSITORY")));
}
