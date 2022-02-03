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

fn get_workspaces(mut cx: FunctionContext) -> JsResult<JsString> {
  let cwd_arg: String = cx.argument::<JsString>(0)?.value(&mut cx);
  let cwd = PathBuf::from(cwd_arg).into_os_string();

  let mut pkg_jsons: Vec<PathBuf> = vec![];
  
  // TODO: Actually find the root package.json file here
  let os_string = Path::new(&cwd).join("packages/*/package.json").into_os_string();
  let pkg_glob = os_string.to_str().unwrap();
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
  cx.export_function("getWorkspaces", get_workspaces)?;  
  Ok(())
}