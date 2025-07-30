use std::path::{Path, PathBuf};
use std::process::Command;
use crate::func;
use crate::kargo::ArgsError;
use std::fs;

pub async fn new(cwd: &PathBuf, flags: Vec<String>) -> Result<(), ArgsError> {
   let status = Command::new("cargo")
      .arg("init")
      .args(&flags)
      .status();
   
   match status {
      Ok(code) if code.success() => {
         let crate_name_flag = flags.iter()
            .find(|f| !f.starts_with('-'))
            .cloned();
         
         let crate_name = crate_name_flag.clone()
            .or_else(|| cwd.file_name().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "(unknown crate)".to_string());
         
         let in_place = crate_name_flag.is_none();
         post_process(&crate_name, in_place);
         Ok(())
      }
      Ok(code) => Err(ArgsError::FuncError(format!(
         "'cargo init' failed: {}",
         code
      ))),
      Err(e) => Err(ArgsError::FuncError(format!(
         "failed to run 'cargo init': {}",
         e
      ))),
   }
}


fn post_process(crate_name: &str, in_place: bool) {
   use std::fs;
   
   let crate_path = if in_place {
      PathBuf::from(".")
   } else {
      PathBuf::from(crate_name)
   };
   
   let cargo_toml = crate_path.join("Cargo.toml");
   let readme = crate_path.join("README.md");
   let license = crate_path.join("LICENSE");
   let gitignore = crate_path.join(".gitignore");
   
   let metadata = format!(
      "authors = [\"Kono\"]\ndescription = \"blank Kargo description.\"\nlicense = \"MIT\"\nreadme = \"README.md\"\nrepository = \"https://github.com/Kono-o/{}\"\nkeywords = [\"\"]\ncategories = [\"\"]\nhomepage = \"https://github.com/Kono-o/{}\"\ndocumentation = \"https://docs.rs/{}\"",
      crate_name, crate_name, crate_name
   );
   
   let profile = "\n\n[profile.release]\nstrip = true\nopt-level = \"z\"\nlto = true\n";
   
   if let Ok(content) = fs::read_to_string(&cargo_toml) {
      if let Some(index) = content.find("[dependencies]") {
         let (head, tail) = content.split_at(index);
         let new_content = format!("{}\n{}\n{}{}", head.trim_end(), metadata, tail.trim_start(), profile);
         let _ = fs::write(&cargo_toml, new_content);
      }
   }
   
   let readme_content = format!(
      "# {}\n\nblank Kargo description.\n\n## Usage\n\n```\nblank\n```\n\n### Examples\n\n```\nblank\n```\n\n### Help\n\n```\nblank\n```\n\n## Install\n\n### From crates.io\n\n```\ncargo install {}\n```\n\n### From source\n\n```\ngit clone https://github.com/Kono-o/{}\ncd {}\ncargo install --path .\n```\n",
      crate_name, crate_name, crate_name, crate_name
   );
   let _ = fs::write(&readme, readme_content);
   
   let mit_license = "MIT License\n\nCopyright (c) 2025 Kono\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in\nall copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN\nTHE SOFTWARE.\n";
   let _ = fs::write(&license, mit_license);
   
   let _ = Command::new("sh")
      .arg("-c")
      .arg(format!("echo '.idea' >> {}", gitignore.display()))
      .status();
   
   func::msg_ok(&format!("kargo post process applied to crate '{}'!", crate_name));
}

pub async fn wrk(_x: &PathBuf, lib: &String, bin: &String) {
   let _ = Command::new("kargo").args(["new", "--lib", lib]).status();
   let _ = Command::new("kargo").args(["new", "--bin", bin]).status();
   
   let _ = fs::write("Cargo.toml", format!(
      "[workspace]\nmembers = [\"{}\", \"{}\"]\nresolver = \"2\"\n",
      lib, bin
   ));
   
   let _ = fs::write("rustfmt.toml", "tab_spaces = 3\n");
   
   let bin_toml = Path::new(bin).join("Cargo.toml");
   
   if let Ok(content) = fs::read_to_string(&bin_toml) {
      let mut lines: Vec<&str> = content.lines().collect();
      let dep_header_index = lines.iter().position(|line| line.trim() == "[dependencies]");
      
      let dep_append = format!(
         "{} = {{ path = \"../{}\" }}\n# use this instead when compiling from github\n# {} = {{ git = \"https://github.com/Kono-o/{}.git\" }}\n# or just crates io link: https://crates.io/crates/{}",
         lib, lib, lib, lib, lib
      );
      
      let new_content = if let Some(index) = dep_header_index {
         lines.insert(index + 1, &dep_append);
         lines.join("\n")
      } else {
         format!("{}\n\n[dependencies]\n{}", content.trim_end(), dep_append)
      };
      
      let _ = fs::write(&bin_toml, new_content);
   }
   
   func::msg_ok("WS: KARGO workspace scaffolded");
}


pub async fn reload() {
   let _ = Command::new("cargo").args(["build","--release"]).status();
   func::msg_ok("kargo crate built!");
   let _ = Command::new("cargo").args(["install", "--path", "."]).status();
   func::msg_ok("kargo crate installed!");
}
