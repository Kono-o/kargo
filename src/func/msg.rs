const RESET: &str = "\x1b[0m";

const OK: &str = "\x1b[1;92m"; //bright green
const ERR: &str = "\x1b[1;91m"; //bright red
const INFO: &str = "\x1b[1;97m"; //bright white

pub fn msg_info(msg: &str) {
   println!("{INFO}{msg}{RESET}");
}
pub fn msg_ok(msg: &str) {
   println!("{OK}{msg}{RESET}");
}
pub fn msg_error(msg: &str) {
   println!("{ERR}{msg}{RESET}");
}