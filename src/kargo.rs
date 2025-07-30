use crate::func;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ArgsError {
   InvalidArgs,
   MissingArgs,
   FuncError(String),
}

impl ArgsError {
   pub fn handle(&self) {
      match self {
         ArgsError::MissingArgs => func::msg_error("missing arguments, run 'kargo help' for info."),
         ArgsError::InvalidArgs => func::msg_error("invalid arguments, run 'kargo help' for info."),
         ArgsError::FuncError(e) => func::msg_error(&format!("{}", e)),
      }
   }
}

#[derive(Debug)]
pub enum Func {
   Help,
   Info,
   New(Vec<String>),
   Wrk { lib: String, bin: String },
   Rld,
}

#[derive(Debug)]
pub struct Args {
   pub dir: PathBuf,
   pub func: Func,
}

pub async fn run() {
   match parse() {
      Err(err) => err.handle(),
      Ok(args) => match func(args).await {
         Err(err) => err.handle(),
         Ok(_) => (),
      },
   }
}

fn parse() -> Result<Args, ArgsError> {
   let mut args = std::env::args().skip(1);
   let token_0 = args.next();
   let flags: Vec<String> = args.collect();
   
   match token_0.as_deref() {
      Some(func) => {
         match func {
            "help"| "h" => Ok(Args { dir: std::env::current_dir().unwrap(), func: Func::Help }),
            "info"| "i" => Ok(Args { dir: std::env::current_dir().unwrap(), func: Func::Info }),
            "rld"| "reload"| "r" => Ok(Args { dir: std::env::current_dir().unwrap(), func: Func::Rld }),
            "new"| "n" | "make"| "mk"| "init" |"create" | "crate" | "cr" => Ok(Args {
               dir: std::env::current_dir().unwrap(),
               func: Func::New(flags),
            }),
            "ws"|"workspace"|"wrk" => {
               if flags.len() < 2 {
                  return Err(ArgsError::MissingArgs);
               }
               let lib = flags[0].clone();
               let bin = flags[1].clone();
               Ok(Args {
                  dir: std::env::current_dir().unwrap(),
                  func: Func::Wrk { lib, bin },
               })
         }
            _ => Err(ArgsError::InvalidArgs)
         }
      }
      _ => Err(ArgsError::InvalidArgs),
   }
}

async fn func(args: Args) -> Result<(), ArgsError> {
   match args.func {
      Func::Help => func::help(),
      Func::Info => func::info(),
      Func::New(flags) => match func::mk(&args.dir, flags).await {
         Err(e) => e.handle(),
         Ok(_) => return Ok(()),
      }
      Func::Wrk { lib, bin } => func::wrk(&args.dir, &lib, &bin).await,
      Func::Rld => func::reload().await,
   }
   Ok(())
   
}
