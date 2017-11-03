To reproduce cross compile bug.

 - Create Docker image

	`docker build -t noyez/dockcore-armv7 .`

 - Run bash in docker image

    `docker run -ti -v $PWD:/work noyez/dockcore-armv7 bash`

 - From bash shell run cargo to build project

    `gosu root cargo build --target=armv7-unknown-linux-gnueabihf`

```
	error: failed to run custom build command for `backtrace-sys v0.1.16`
   	process didn't exit successfully: `/work/target/debug/build/backtrace-sys-514698a0155fc278/build-script-build` (exit code: 101)
    --- stdout
    OPT_LEVEL = Some("0")
    TARGET = Some("x86_64-unknown-linux-gnu")
    HOST = Some("x86_64-unknown-linux-gnu")
    TARGET = Some("x86_64-unknown-linux-gnu")
    TARGET = Some("x86_64-unknown-linux-gnu")
    HOST = Some("x86_64-unknown-linux-gnu")
    CC_x86_64-unknown-linux-gnu = None
    CC_x86_64_unknown_linux_gnu = None
    HOST_CC = None
    CC = Some("/usr/bin/arm-linux-gnueabihf-gcc")
    TARGET = Some("x86_64-unknown-linux-gnu")
    HOST = Some("x86_64-unknown-linux-gnu")
    CFLAGS_x86_64-unknown-linux-gnu = None
    CFLAGS_x86_64_unknown_linux_gnu = None
    HOST_CFLAGS = None
    CFLAGS = None
    DEBUG = Some("true")
    running: "sh" "/opt/rust/registry/src/github.com-1ecc6299db9ec823/backtrace-sys-0.1.16/src/libbacktrace/configure" "--with-pic" "--disable-multilib" "--disable-shared" "--disable-host-shared" "--host=x86_64-unknown-linux-gnu" "--build=x86_64-unknown-linux-gnu"
    checking build system type... x86_64-unknown-linux-gnu
    checking host system type... x86_64-unknown-linux-gnu
    checking target system type... x86_64-unknown-linux-gnu
    checking for x86_64-unknown-linux-gnu-gcc... /usr/bin/arm-linux-gnueabihf-gcc
    checking for C compiler default output file name...

    --- stderr
    configure: error: in `/work/target/debug/build/backtrace-sys-c8a25c9ec23a59ad/out':
    configure: error: C compiler cannot create executables
    See `config.log' for more details.
    thread 'main' panicked at 'failed with: exit code: 77', /opt/rust/registry/src/github.com-1ecc6299db9ec823/backtrace-sys-0.1.16/build.rs:176:8
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
