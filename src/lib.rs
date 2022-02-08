use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use glob::glob;
use neon::prelude::*;
use rayon::prelude::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
  dir: std::path::PathBuf,
  packageJson: serde_json::Value
}

fn glob_require(mut cx: FunctionContext) -> JsResult<JsString> {
  let pkg_glob: String = cx.argument::<JsString>(0)?.value(&mut cx);
  let mut pkg_jsons: Vec<PathBuf> = vec![];
  
  glob(&pkg_glob).expect("globbing failed")
    .map(|path| path.expect("globbing failed"))    
    .for_each(|entry| {
      pkg_jsons.push(entry);
    });
  
  let pkg_map: Vec<Workspace> = pkg_jsons.into_par_iter()
    .map(|entry| {
      let pkg_path = Path::new(&entry);
      let file = File::open(pkg_path).expect("reader issue");
      
      let json: serde_json::Value = serde_json::from_reader(file).expect("json parsing");
      
      let pkg = Workspace { dir: entry, packageJson: json };
      pkg
    }).collect();
  
  let str = serde_json::to_string(&pkg_map).unwrap();

  Ok(cx.string(str))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("globRequire", glob_require)?;  
  Ok(())
}