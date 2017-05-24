// GENERATED by gen_stat.py

use std::fs;

pub struct Stat {
    pub uid: u32,
    pub gid: u32,
    pub ctime: i64,
    pub ctime_nano: i64,
}

impl Stat {

    #[cfg(target_os = "android")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::android::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "freebsd")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::freebsd::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "haiku")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::haiku::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "ios")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::ios::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::linux::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "netbsdosx")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::netbsdosx::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(target_os = "solaris")]
    pub fn from(meta: &fs::Metadata) -> Stat {
        use std::os::solaris::fs::MetadataExt;
        Stat {
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
        }
    }

    #[cfg(not(any(target_os = "android",
                  target_os = "freebsd",
                  target_os = "haiku",
                  target_os = "ios",
                  target_os = "linux",
                  target_os = "netbsdosx",
                  target_os = "solaris")))]
    pub fn from(meta: &fs::Metadata) -> Stat {
        Stat {
            uid: 0,
            gid: 0,
            ctime: 0,
            ctime_nano: 0,
        }
    }
}

