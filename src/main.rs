// Implement the file system kernel module
use fuser::{Request, KernelConfig};//Context, DirEntry, File, Inode, Result};
use std::env;
use core::ffi::c_int;

struct UserProcFS {
    // Initialize the UserProcFS
}

impl UserProcFS {
    fn new() -> Self {
        UserProcFS {
            // Initialize the UserProcFS
        }
    }

    fn unmount() {
        // Unmount the file system
    }
}

impl fuser::Filesystem for UserProcFS {
    fn init(&mut self, _req: &Request, _config: &mut KernelConfig) -> Result<(), c_int> {
        // Initialize the fs
        Ok(())
    }/*

    fn getattr(&self, _ctx: &Context, _path: &str, _attr: &mut fuser::stat::Stat) -> Result<()> {
        // Get the file attributes
        Ok(())
    }

    fn readdir(&self, _ctx: &Context, _path: &str, _entry: &mut fuser::DirEntry) -> Result<()> {
        // Read the directory contents
        Ok(())
    }

    fn read(&self, _ctx: &Context, _path: &str, _offset: u64, _length: u64, _buf: &mut [u8]) -> Result<()> {
        // Read the file contents
        Ok(())
    }

    fn write(&self, _ctx: &Context, _path: &str, _offset: u64, _length: u64, _buf: &[u8]) -> Result<()> {
        // Write to the file
        Ok(())
    }

    fn create(&self, _ctx: &Context, _path: &str, _mode: u32, _attr: &mut fuser::stat::Stat) -> Result<()> {
        // Create a new file
        Ok(())
    }

    fn unlink(&self, _ctx: &Context, _path: &str) -> Result<()> {
        // Delete a file
        Ok(())
    }

    fn rmdir(&self, _ctx: &Context, _path: &str) -> Result<()> {
        // Delete a directory
        Ok(())
    }

    fn rename(&self, _ctx: &Context, _old_path: &str, _new_path: &str) -> Result<()> {
        // Rename a file or directory
        Ok(())
    }

    fn statfs(&self, _ctx: &Context, _path: &str, _buf: &mut fuser::stat::Statfs) -> Result<()> {
        // Get the file system statistics
        Ok(())
    }*/
}

fn main() {
    use::std::sync::Arc;
    // Threads handle
    let mut handles = Vec::new();

    // Create a new instance of the file system
    let fs = UserProcFS::new();

    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {:?} <MOUNTPOINT>", env::args().nth(1));
            return;
        }
    };

    println!("Attempting to mount the UserProcFS file system at: {:?}", mountpoint);
    let mounted = fuser::spawn_mount2(fs, mountpoint, &[fuser::MountOption::RW]);
    handles.push(mounted);
    
    println!("Sleeping 8 seconds before umounting.");
    std::thread::sleep(std::time::Duration::from_secs(8));
    
    println!("Attempting to unmount the UserProcFS file system");
    UserProcFS::unmount();

    // Wait for all threads to finish
    for handle in handles {
        handle.expect("Failed").join();
    }
}

