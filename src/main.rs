// extern crate termion;

use std::process::Command;
use std::env;
use regex::Regex;
// use termion::color;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut current_step: usize = 0;

  if args.len() == 1 {
    println!("Please provide valid Semantic Version");

  } else {
    let mut iter = ["major", "minor", "patch", "premajor", "preminor", "prepatch", "prerelease"].iter();
    let version = &args[1];

    match iter.find(|&&x| x == version) {
      Some(_ver) => {
        if npm_build() {
          current_step += 1;

          if npm_version(version) {
            current_step += 1;

            if git_push() {
              current_step += 1;
              if npm_publish() {
                println!("packaged successfully published");
              } else {
                failure(current_step);
              }

            } else {
              failure(current_step);
            }

          } else {
            failure(current_step);
          }

        } else {
          failure(current_step);
        }
      },
      None => {
        println!("{} isn't a valid version", &version);
      }
    };
  }
}

fn npm_build() -> bool {
  let status = Command::new("npm")
    .arg("run")
    .arg("build")
    .status()
    .expect("npm build failed");

  return status.success();
}

fn npm_version(version: &str) -> bool {
  let status = Command::new("npm")
    .arg("version")
    .arg(version)
    .status()
    .expect("npm version failed");

  return status.success();
}

fn git_push() -> bool {
  let output = Command::new("git")
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()
    .expect("do something");

  let re     = Regex::new(r"\n").expect("regex broken");
  let stdout = String::from_utf8_lossy(&output.stdout);
  let branch = re.replace_all(&stdout, "");

  let status = Command::new("git")
    .args(&["push", "origin", &branch])
    .status()
    .expect("git push failed");

  return status.success();
}

fn npm_publish() -> bool {
  let status = Command::new("npm")
    .arg("publish")
    .status()
    .expect("publish failed");
  return status.success();
}

fn failure(step: usize) {
  let steps = [ "build", "version", "push", "publish"];
  println!("Failed on {} step", steps[step]);
}
