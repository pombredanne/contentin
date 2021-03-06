#!/usr/bin/env python3

OSES = [
    'android',
    'freebsd',
    'haiku',
    'ios',
    'linux',
    'netbsd'
    'osx',
    'solaris',
]

F = """// GENERATED by gen_stat.py

use std::fs;

pub struct Stat {
    pub uid: u32,
    pub gid: u32,
    pub ctime: i64,
    pub ctime_nano: i64,
    pub mode: u32,
}

impl Stat {
"""

for os in OSES:
    F += """
    #[cfg(target_os = "{0}")]
    pub fn from(meta: &fs::Metadata) -> Stat {{
        use std::os::{0}::fs::MetadataExt;
        Stat {{
            uid: meta.st_uid(),
            gid: meta.st_gid(),
            ctime: meta.st_ctime(),
            ctime_nano: meta.st_ctime_nsec(),
            mode: meta.st_mode(),
        }}
    }}
""".format(os)

F += """
    #[cfg(not(any({0})))]
    pub fn from(meta: &fs::Metadata) -> Stat {{
        Stat {{
            uid: 0,
            gid: 0,
            ctime: 0,
            ctime_nano: 0,
            mode: 0,
        }}
    }}
}}
""".format(',\n                  '.join('target_os = "{}"'.format(os) for os in OSES))

print(F)
