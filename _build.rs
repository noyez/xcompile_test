
// Taken from backtrace-rs/build.rs


extern crate cc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use std::fs::File;
use std::io::prelude::*;


macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn try_tool(compiler: &cc::Tool, cc: &str, compiler_suffix: &str, tool_suffix: &str)
            -> Option<PathBuf> {
    if !cc.ends_with(compiler_suffix) {
        return None
    }
    let cc = cc.replace(compiler_suffix, tool_suffix);
    let candidate = compiler.path().parent().unwrap().join(cc);
    if Command::new(&candidate).output().is_ok() {
        Some(candidate)
    } else {
        None
    }
}

fn find_tool(compiler: &cc::Tool, cc: &str, tool: &str) -> PathBuf {
    // Allow overrides via env var
    if let Some(s) = env::var_os(tool.to_uppercase()) {
        return s.into()
    }
    let tool_suffix = format!("-{}", tool);
    try_tool(compiler, cc, "-gcc", &tool_suffix)
        .or_else(|| try_tool(compiler, cc, "-clang", &tool_suffix))
        .or_else(|| try_tool(compiler, cc, "-cc", &tool_suffix))
        .unwrap_or_else(|| PathBuf::from(tool))
}

fn main() {
   // env::set_var("TARGET", "armv7-unknown-linux-gnueabihf");
   // env::set_var("HOST", "armv7-unknown-linux");
    let src = env::current_dir().unwrap();
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or(String::from("NO_TARGET"));
    let host = env::var("HOST").unwrap_or(String::from("NO_HOST"));
    let build = env::var("BUILD").unwrap_or(String::from("NO_BUILD"));

    let configure = src.join("env_test.sh").into_os_string();
    //let configure = src.join("configure").into_os_string();

    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();
    let cc = compiler.path().file_name().unwrap().to_str().unwrap();
    let mut flags = OsString::new();
    for (i, flag) in compiler.args().iter().enumerate() {
        if i > 0 {
            flags.push(" ");
        }
        flags.push(flag);
    }
    let ar = find_tool(&compiler, cc, "ar");
    let ranlib = find_tool(&compiler, cc, "ranlib");
    let mut cmd = Command::new("sh");
    // We will iterate through the references to the element returned by
    // env::vars();
    let mut contents = String::new();
    for (key, value) in env::vars() {
        contents.push_str(&*(format!(" -- {}: {}\n", key, value)));
    }

    for i in vec!["TARGET", "HOST", "BUILD"] {
        contents.push_str(&*(format!(" -- -- {}: {:?}\n", i, env::var(i))));
    }


    let mut file = File::create("build-rs-debug.txt").unwrap();
    file.write_all(contents.as_bytes());

    cmd.arg(configure)
       .current_dir(&dst)
       .env("AR", &ar)
       .env("RANLIB", &ranlib)
       .env("CC", compiler.path())
       .env("CFLAGS", flags)
       .arg("--with-pic")
       .arg("--disable-multilib")
       .arg("--disable-shared")
       .arg("--disable-host-shared")
       .arg(format!("--target={}", "armv7-unknown-linux-gnueabihf"));
       //.arg(format!("--host={}", "arm-linux"));

    // Apparently passing this flag causes problems on Windows
    if !host.contains("windows") {
       cmd.arg(format!("--build={}", build));
    }

    run(&mut cmd, "sh");
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(s) => s,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!("\n\nfailed to execute command: {}\nIs `{}` \
                    not installed?\n\n",
                   e,
                   program);
        }
        Err(e) => panic!("failed to get status: {}", e),
    };
    if !status.success() {
        panic!("failed with: {}", status);
    }
}
