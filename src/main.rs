// Implement the file system kernel module
use fuser::{Request, KernelConfig};
use std::env;
use std::path::PathBuf;
use std::ffi::OsStr;
use core::ffi::c_int;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use fuser::FileType as FType;

struct File<'a> {
    name: &'a OsStr,
    ty: fuser::FileType,
    inode: u64,
    //content: Vec<UserProcess>, ou Temperature ou etc.
    // Design ideas: one File struct and one Dir struct which both implement a func to compute
    // its size. And also a field with its content, which is a Vec of its representing struct?
}

impl<'a> File<'a> {
    fn new(name: &'a OsStr, ty: fuser::FileType, inode: u64) -> Self {
        File {
            name,
            ty,
            inode,
            //content,
        }
    }

    fn get_perms(&self) -> u16 {
        // Get the file permissions

        match self.ty {
            fuser::FileType::Directory => 0o755,
            fuser::FileType::RegularFile => 0o644,
            _ => 0o000,
        }
    }
}

struct UserProcFS {
    // Initialize the UserProcFS
    
    // Mountpoint
    mountpoint: PathBuf,

    // Files
    files: Vec<File<'static>>,
}

impl UserProcFS {
    fn new(mountpoint: PathBuf) -> Self {
        // Initialize the UserProcFS
        let mut new_userprocfs = UserProcFS {
            mountpoint: mountpoint,
            files: Vec::new()
        };
        new_userprocfs.files.push(File::new(OsStr::new("."), FType::Directory, 1));
        new_userprocfs.files.push(File::new(OsStr::new(".."), FType::Directory, 2));
        new_userprocfs.files.push(File::new(OsStr::new("processes"), FType::Directory, 3));
        new_userprocfs.files.push(File::new(OsStr::new("temperatures"), FType::RegularFile, 4));
        new_userprocfs.files.push(File::new(OsStr::new("memory"), FType::RegularFile, 5));
        new_userprocfs.files.push(File::new(OsStr::new("network"), FType::RegularFile, 6));
        new_userprocfs.files.push(File::new(OsStr::new("disk"), FType::RegularFile, 7));
        new_userprocfs.files.push(File::new(OsStr::new("cpu"), FType::RegularFile, 8));
        new_userprocfs
    }

    fn get_file_by_inode(&self, inode: u64) -> Option<&File> {
        // Get the file by inode

        match inode {
            1 => Some(&self.files[0]),
            2 => Some(&self.files[1]),
            3 => Some(&self.files[2]),
            4 => Some(&self.files[3]),
            5 => Some(&self.files[4]),
            6 => Some(&self.files[5]),
            7 => Some(&self.files[6]),
            8 => Some(&self.files[7]),
            _ => None,
        }
    }

    fn get_file_by_name(&self, name: &OsStr) -> Option<&File> {
        // Get the file by name

        if name == "processes" {
            Some(&self.files[0])
        } else if name == "temperatures" {
            Some(&self.files[1])
        } else if name == "memory" {
            Some(&self.files[2])
        } else if name == "network" {
            Some(&self.files[3])
        } else if name == "disk" {
            Some(&self.files[4])
        } else if name == "cpu" {
            Some(&self.files[5])
        } else {
            None
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
    }

    fn getattr(&mut self, _req: &Request, ino: u64, fh: Option<u64>, reply: fuser::ReplyAttr) {
        // Get the file attributes
        
        let file = UserProcFS::get_file_by_inode(&self, ino);

        match file {
            Some(file) => {
                let now = SystemTime::now();
                reply.attr(&Duration::from_secs(1), &fuser::FileAttr {
                    ino: file.inode,
                    size: 0,
                    blocks: 0,
                    atime: now,
                    mtime: now,
                    ctime: now,
                    crtime: now,
                    kind: file.ty,
                    perm: file.get_perms(),
                    nlink: 2,
                    uid: 1000,
                    gid: 1000,
                    rdev: 0,
                    blksize: 512,
                    flags: 0,
                });
            },
            None => {
                reply.error(libc::ENOENT);
            }
        }
    }

    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: fuser::ReplyEntry) {
        // Look up a directory entry
      
        let file = UserProcFS::get_file_by_name(&self, name);

        match file {
            Some(file) => {
                let now = SystemTime::now();
                reply.entry(&Duration::from_secs(1), &fuser::FileAttr {
                    ino: file.inode,
                    size: 0,
                    blocks: 0,
                    atime: now,
                    mtime: now,
                    ctime: now,
                    crtime: now,
                    kind: file.ty,
                    perm: file.get_perms(),
                    nlink: 2,
                    uid: 1000,
                    gid: 1000,
                    rdev: 0,
                    blksize: 512,
                    flags: 0,
                }, 0);
            },
            None => {
                reply.error(libc::ENOENT);
            }
        }
    }

    fn readdir(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, mut reply: fuser::ReplyDirectory) {
        // Read the directory contents

        if ino != 1 && ino != 2 {
            reply.error(libc::ENOENT);
            return;
        }

        for file in &self.files {
            if reply.add(file.inode, offset + 1, file.ty, file.name) {
                println!("Failed to add entry to directory, buffer is full.");
                break;
            }
        }

        reply.ok();
    }

    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        // Open the file

        reply.opened(0, 0);
    }

    fn read(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, size: u32, flags: i32, lock_owner: Option<u64>, reply: fuser::ReplyData) {
        // Read the file contents

        use sysinfo::{System};

        let mut sys = System::new_all();
        sys.refresh_all();

        if ino == 4 {
            let data = format!("{:?}", sys.total_memory()).into_bytes();

            let end = (offset as usize + size as usize).min(data.len());
            reply.data(&data[offset as usize..end]);
        } else {
            reply.error(libc::ENOENT);
        }
    }

    fn write(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, data: &[u8], write_flags: u32, flags: i32, lock_owner: Option<u64>, reply: fuser::ReplyWrite) {
        // Write to the file
    }

    fn create(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, flags: i32, reply: fuser::ReplyCreate) {
        // Create a new file
    }

    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: fuser::ReplyEmpty) {
        // Delete a file
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: fuser::ReplyEmpty) {
        // Delete a directory
    }

    fn rename(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, newparent: u64, newname: &OsStr, flags: u32, reply: fuser::ReplyEmpty) {
        // Rename a file or directory
    }

    fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: fuser::ReplyStatfs) {
        // Get the file system statistics
    }
}

fn main() {
    // Threads handle
    let mut handles = Vec::new();

    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {:?} <MOUNTPOINT>", env::args().nth(1));
            return;
        }
    };
    // Create a new instance of the file system
    let fs = UserProcFS::new(mountpoint.clone().into());


    println!("Attempting to mount the UserProcFS file system at: {:?}", mountpoint);
    let mounted = fuser::spawn_mount2(fs, mountpoint, &[fuser::MountOption::RW]);
    handles.push(mounted);
    
    println!("Sleeping 8 seconds before umounting.");
    std::thread::sleep(std::time::Duration::from_secs(8));
    // TODO : handle signals to unmount the file system.
    
    println!("Attempting to unmount the UserProcFS file system");
    UserProcFS::unmount();

    // Wait for all threads to finish
    for handle in handles {
        handle.expect("Failed to unmount UserProcFS").join();
        println!("UserProcFS unmounted successfully.");
    }
}

