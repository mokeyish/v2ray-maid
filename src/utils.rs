use std::env;
use std::path::{Path, PathBuf};

pub fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

pub fn pick_free_tcp_port() -> u16 {
    use std::net::TcpListener;
    if let Ok(listener) = TcpListener::bind("127.0.0.1:0") {
        if let Ok(local_addr) = listener.local_addr() {
            return local_addr.port();
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    pick_free_tcp_port()
}
