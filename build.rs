#[allow(unused_variables)]
fn main() {
    // e.g. /home/w/.rustup
    let rustup_home = std::env::var("RUSTUP_HOME").unwrap();
    // e.g. nightly-x86_64-unknown-linux-gnu
    let toolchain = std::env::var("RUSTUP_TOOLCHAIN").unwrap();
    // let host = std::env::var("HOST").unwrap();
    let target = std::env::var("TARGET").unwrap();

    // e.g. /home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
    let path = std::path::Path::new(&rustup_home)
        .join("toolchains")
        .join(toolchain)
        .join("lib")
        .join("rustlib")
        .join(target)
        .join("lib");
    if !path.exists() {
        panic!("{:?} not exist!\nrequire nightly toolchain with rustc-dev component", path);
    }
    
    let dylib_path_1 = "/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib";
    // dylib_path_2 NOT WORKING!
    let dylib_path_2 = "/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib";
    println!("cargo:rustc-link-search=all={}", path.to_str().unwrap());

    for (k,v) in std::env::vars() {
        println!("{}={}", k,v);
    }
}

/*
cargo:rustc-link-search="/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib"
BREAKPAD_DUMP_LOCATION=/home/w/.config/Code/exthost Crash Reports
BROWSER=/usr/bin/firefox
CARGO=/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo
CARGO_CFG_FEATURE=cargo-clippy
CARGO_CFG_PANIC=unwind
CARGO_CFG_TARGET_ABI=
CARGO_CFG_TARGET_ARCH=x86_64
CARGO_CFG_TARGET_ENDIAN=little
CARGO_CFG_TARGET_ENV=gnu
CARGO_CFG_TARGET_FAMILY=unix
CARGO_CFG_TARGET_FEATURE=adx,aes,avx,avx2,avx512vaes,avx512vpclmulqdq,bmi1,bmi2,cmpxchg16b,f16c,fma,fxsr,lzcnt,movbe,pclmulqdq,popcnt,rdrand,rdseed,sha,sse,sse2,sse3,sse4.1,sse4.2,sse4a,ssse3,xsave,xsavec,xsaveopt,xsaves
CARGO_CFG_TARGET_HAS_ATOMIC=16,32,64,8,ptr
CARGO_CFG_TARGET_HAS_ATOMIC_EQUAL_ALIGNMENT=16,32,64,8,ptr
CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE=16,32,64,8,ptr
CARGO_CFG_TARGET_OS=linux
CARGO_CFG_TARGET_POINTER_WIDTH=64
CARGO_CFG_TARGET_THREAD_LOCAL=
CARGO_CFG_TARGET_VENDOR=unknown
CARGO_CFG_UNIX=
CARGO_ENCODED_RUSTFLAGS=-Clink-arg=-fuse-ld=lld-Ctarget-cpu=znver3
CARGO_HOME=/home/w/.cargo
CARGO_MAKEFLAGS=-j --jobserver-fds=5,6 --jobserver-auth=5,6
CARGO_MANIFEST_DIR=/home/w/temp/rustc_driver_trait
CARGO_PKG_AUTHORS=
CARGO_PKG_DESCRIPTION=
CARGO_PKG_HOMEPAGE=
CARGO_PKG_LICENSE=
CARGO_PKG_LICENSE_FILE=
CARGO_PKG_NAME=rustc_driver_trait
CARGO_PKG_REPOSITORY=
CARGO_PKG_VERSION=0.1.0
CARGO_PKG_VERSION_MAJOR=0
CARGO_PKG_VERSION_MINOR=1
CARGO_PKG_VERSION_PATCH=0
CARGO_PKG_VERSION_PRE=
CHROME_DESKTOP=code-url-handler.desktop
CLIPPY_ARGS=
DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
DEBUG=true
DESKTOP_SESSION=plasma
DISPLAY=:0
EDITOR=/usr/bin/nano
ELECTRON_RUN_AS_NODE=1
GDK_BACKEND=x11
GTK2_RC_FILES=/etc/gtk-2.0/gtkrc:/home/w/.gtkrc-2.0:/home/w/.config/gtkrc-2.0
GTK_IM_MODULE=fcitx5
GTK_MODULES=canberra-gtk-module
GTK_RC_FILES=/etc/gtk/gtkrc:/home/w/.gtkrc:/home/w/.config/gtkrc
HOME=/home/w
HOST=x86_64-unknown-linux-gnu
KDE_APPLICATIONS_AS_SCOPE=1
KDE_FULL_SESSION=true
KDE_SESSION_UID=1000
KDE_SESSION_VERSION=5
LANG=en_US.UTF-8
LARCH_PATH=/usr/share/splint/lib
LCLIMPORTDIR=/usr/share/splint/imports
LC_ADDRESS=en_US.UTF-8
LC_IDENTIFICATION=en_US.UTF-8
LC_MEASUREMENT=en_US.UTF-8
LC_MONETARY=en_US.UTF-8
LC_NAME=en_US.UTF-8
LC_NUMERIC=en_US.UTF-8
LC_PAPER=en_US.UTF-8
LC_TELEPHONE=en_US.UTF-8
LC_TIME=en_US.UTF-8
LD_LIBRARY_PATH=/home/w/temp/rustc_driver_trait/target/debug/deps:/home/w/temp/rustc_driver_trait/target/debug:/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib
LOGNAME=w
MAIL=/var/spool/mail/w
MOTD_SHOWN=pam
NO_AT_BRIDGE=1
NUM_JOBS=24
OPT_LEVEL=0
ORIGINAL_XDG_CURRENT_DESKTOP=KDE
OUT_DIR=/home/w/temp/rustc_driver_trait/target/debug/build/rustc_driver_trait-419e86817c3c2f09/out
PATH=/home/w/.cargo/bin:/home/w/.cargo/bin:/home/w/.cargo/bin:/home/w/.local/bin:/home/w/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/lib/jvm/default/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/usr/lib/jvm/default/bin
PROFILE=debug
PWD=/home/w
QT_AUTO_SCREEN_SCALE_FACTOR=0
QT_IM_MODULE=fcitx5
QT_LINUX_ACCESSIBILITY_ALWAYS_ON=1
QT_SCREEN_SCALE_FACTORS=DVI-D-0=1.25;HDMI-0=1.25;
RUSTC=rustc
RUSTC_WORKSPACE_WRAPPER=/home/w/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/clippy-driver
RUSTDOC=rustdoc
RUSTUP_HOME=/home/w/.rustup
RUSTUP_TOOLCHAIN=nightly-x86_64-unknown-linux-gnu
RUST_BACKTRACE=short
RUST_RECURSION_COUNT=3
SESSION_MANAGER=local/ww:@/tmp/.ICE-unix/861,unix/ww:/tmp/.ICE-unix/861
SHELL=/bin/bash
SHLVL=0
SSH_AGENT_PID=730
SSH_AUTH_SOCK=/tmp/ssh-XXXXXXbXCWe3/agent.729
SSL_CERT_DIR=/etc/ssl/certs
SSL_CERT_FILE=/etc/ssl/cert.pem
SYSTEMD_EXEC_PID=693
TARGET=x86_64-unknown-linux-gnu
USER=w
VSCODE_AMD_ENTRYPOINT=vs/workbench/services/extensions/node/extensionHostProcess
VSCODE_CODE_CACHE_PATH=/home/w/.config/Code/CachedData/7f6ab5485bbc008386c4386d08766667e155244e
VSCODE_CRASH_REPORTER_START_OPTIONS={"companyName":"Microsoft","productName":"VSCode","submitURL":"appcenter://code?aid=fba07a4d-84bd-4fc8-a125-9640fc8ce171&uid=24293b22-d2c2-42c1-bdfd-ca3ab2be9df2&iid=24293b22-d2c2-42c1-bdfd-ca3ab2be9df2&sid=24293b22-d2c2-42c1-bdfd-ca3ab2be9df2","uploadToServer":true}
VSCODE_CWD=/home/w
VSCODE_HANDLES_UNCAUGHT_ERRORS=true
VSCODE_IPC_HOOK=/run/user/1000/vscode-8f9d6e05-1.60.2-main.sock
VSCODE_IPC_HOOK_EXTHOST=/run/user/1000/vscode-ipc-cb0899ab-51f5-4c2c-9b48-d26408914c44.sock
VSCODE_LOG_NATIVE=false
VSCODE_LOG_STACK=false
VSCODE_NLS_CONFIG={"locale":"en-us","availableLanguages":{},"_languagePackSupport":true}
VSCODE_PID=2717
VSCODE_PIPE_LOGGING=true
VSCODE_VERBOSE_LOGGING=true
XAUTHORITY=/home/w/.Xauthority
XCURSOR_SIZE=24
XCURSOR_THEME=breeze_cursors
XDG_CONFIG_DIRS=/home/w/.config/kdedefaults:/etc/xdg
XDG_CURRENT_DESKTOP=KDE
XDG_RUNTIME_DIR=/run/user/1000
XDG_SEAT=seat0
XDG_SEAT_PATH=/org/freedesktop/DisplayManager/Seat0
XDG_SESSION_CLASS=user
XDG_SESSION_DESKTOP=KDE
XDG_SESSION_ID=1
XDG_SESSION_PATH=/org/freedesktop/DisplayManager/Session0
XDG_SESSION_TYPE=x11
XDG_VTNR=1
XMODIFIERS=@im=fcitx5
_=/opt/visual-studio-code/code
*/
