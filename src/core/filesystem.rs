use std::fs;
use std::path::PathBuf;
use nix::unistd;
use nix::sys::stat;
use nix::Error;
use nix::mount;

static ROOT_PATH: &str = "/";
static OLD_ROOT_PATH: &str = "oldroot";
static PROC: &str = "proc";

pub fn mount_proc(){
	const NONE: Option<&'static [u8]> = None;
	mount::mount(Some(PROC), PROC, Some(PROC), mount::MsFlags::empty(), NONE).expect("Failed to mount the /proc");

}
pub fn mount_rootfs(rootfs: &str) -> Result<(), nix::Error> {
    // https://man7.org/linux/man-pages/man2/pivot_root.2.html
    mount::mount(
        None::<&str>,
        "/",
        None::<&str>,
        mount::MsFlags::MS_PRIVATE | mount::MsFlags::MS_REC,
        None::<&str>,
    );

    mount::mount(
        Some(rootfs),
        rootfs,
        None::<&str>,
        mount::MsFlags::MS_BIND | mount::MsFlags::MS_REC,
        None::<&str>,
    );

    Ok(())
}

// pub fn unmount_host_root_fs(){
// 	let _status = mount::umount2(OLD_ROOT_PATH, mount::MntFlags::MNT_DETACH);
// }

pub fn unmount_proc(){
	mount::umount("proc").unwrap();
}

pub fn set_root_fs(rootfs: &str) -> Result<(), nix::Error>{
    unistd::chdir(rootfs)?;
    let p_old_root_fs = PathBuf::from(rootfs).join(OLD_ROOT_PATH);

    unistd::mkdir(
       &p_old_root_fs,
       stat::Mode::S_IRWXU | stat::Mode::S_IRWXG | stat::Mode::S_IRWXO
    );
    unistd::pivot_root(rootfs,  &p_old_root_fs)?;
    mount::umount2(OLD_ROOT_PATH, mount::MntFlags::MNT_DETACH);
    fs::remove_dir_all(&p_old_root_fs);
    unistd::chdir(ROOT_PATH);
    Ok(())
}
