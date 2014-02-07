extern mod native;
extern mod rsfml;
mod control;
mod run;
mod window;

// Macs need rsfml to start on main thread
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
	// Run Program
	::run::main_loop();
}