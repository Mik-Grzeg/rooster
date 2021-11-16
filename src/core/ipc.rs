use nix::{
    sys::socket, unistd::{close, read, write},
};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct IpcParent {
    fd: i32,
    sock_path: String,
}

impl IpcParent {
    pub fn new(path: &String) -> Result<IpcParent> {
        let socket_raw_fd = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::SeqPacket,
            socket::SockFlag::SOCK_CLOEXEC,
            None
        )?;

        let sock_addr = socket::SockAddr::new_unix(Path::new(path))?;
        socket::bind(socket_raw_fd, &sock_addr)?;

        socket::listen(socket_raw_fd, 10)?;
        Ok(IpcParent {
            fd: socket_raw_fd,
            sock_path: path.clone(),
        })
    }

    pub fn wait(&self) -> Result<String> {
        let child_sock = socket::accept(self.fd)?;

        let mut buf = [0; 1024];
        let fd_size = read(child_sock, &mut buf).unwrap();

        Ok(std::str::from_utf8(&buf[0..fd_size])?.trim().to_string())
    }

    pub fn close(&self) -> Result<()> {
        close(self.fd)?;
        std::fs::remove_file(&self.sock_path)?;

        Ok(())
    }
}

pub struct IpcChild {
    fd: i32,
}

impl IpcChild {
    pub fn new(path: &String) -> Result<IpcChild> {
        let socket_raw_fd = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::SeqPacket,
            socket::SockFlag::SOCK_CLOEXEC,
            None
        )?;

        let sock_addr = socket::SockAddr::new_unix(Path::new(path))?;
        socket::connect(socket_raw_fd, &sock_addr)?;

        Ok(IpcChild {
            fd: socket_raw_fd,
        })
    }

    pub fn notify(&self, msg: &String) -> Result<()> {
        write(self.fd, msg.as_bytes())?;
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        close(self.fd)?;
        Ok(())
    }
}
