#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn mymemdump(
    mut fd: *mut FILE,
    mut p: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < len / 16 as libc::c_int {
        fprintf(
            fd,
            b"0x%016lX: \0" as *const u8 as *const libc::c_char,
            (p as libc::c_ulong).wrapping_add((k * 16 as libc::c_int) as libc::c_ulong),
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut c: libc::c_int = *p.offset((k * 16 as libc::c_int + i) as isize)
                as libc::c_int & 0xff as libc::c_int;
            fprintf(fd, b"%02X \0" as *const u8 as *const libc::c_char, c);
            i += 1;
            i;
        }
        fprintf(fd, b" \0" as *const u8 as *const libc::c_char);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            let mut c2: libc::c_int = *p.offset((k * 16 as libc::c_int + j) as isize)
                as libc::c_int & 0xff as libc::c_int;
            fprintf(
                fd,
                b"%c\0" as *const u8 as *const libc::c_char,
                if c2 >= 32 as libc::c_int && c2 <= 127 as libc::c_int {
                    c2
                } else {
                    '.' as i32
                },
            );
            j += 1;
            j;
        }
        fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
        k += 1;
        k;
    }
    if len % 16 as libc::c_int != 0 {
        fprintf(
            fd,
            b"0x%016lX: \0" as *const u8 as *const libc::c_char,
            (p as libc::c_ulong)
                .wrapping_add(
                    (len / 16 as libc::c_int * 16 as libc::c_int) as libc::c_ulong,
                ),
        );
        let mut i2: libc::c_int = len / 16 as libc::c_int * 16 as libc::c_int;
        while i2 < len / 16 as libc::c_int * 16 as libc::c_int + 16 as libc::c_int {
            if i2 < len {
                let mut c3: libc::c_int = *p.offset(i2 as isize) as libc::c_int
                    & 0xff as libc::c_int;
                fprintf(fd, b"%02X \0" as *const u8 as *const libc::c_char, c3);
            } else {
                fprintf(fd, b"   \0" as *const u8 as *const libc::c_char);
            }
            i2 += 1;
            i2;
        }
        fprintf(fd, b" \0" as *const u8 as *const libc::c_char);
        let mut j2: libc::c_int = len / 16 as libc::c_int * 16 as libc::c_int;
        while j2 < len {
            let mut c4: libc::c_int = *p.offset(j2 as isize) as libc::c_int
                & 0xff as libc::c_int;
            fprintf(
                fd,
                b"%c\0" as *const u8 as *const libc::c_char,
                if c4 >= 32 as libc::c_int && c4 <= 127 as libc::c_int {
                    c4
                } else {
                    '.' as i32
                },
            );
            j2 += 1;
            j2;
        }
        fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
