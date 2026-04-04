use crate::cli::{collect_args, cstring_from_os};
use crate::debug::shell::{
    c_readline_with_prompt, fclose_file, fopen_write, free_c_ptr, output_or_stdout, stdout_handle,
    write_file_str, FILE,
};
use crate::foundation::globals::xmlFree;
use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};

const MAX_COMMAND_SIZE: usize = 100;
const MAX_ARG_SIZE: usize = 400;
const XML_SGML_DEFAULT_CATALOG: &str = "/etc/sgml/catalog";

#[repr(C)]
struct xmlCatalog {
    _private: [u8; 0],
}

#[repr(C)]
struct xmlURI {
    _private: [u8; 0],
}

type xmlCatalogPtr = *mut xmlCatalog;
type xmlURIPtr = *mut xmlURI;

unsafe extern "C" {
    fn xmlCatalogSetDebug(level: c_int) -> c_int;
    fn xmlInitializeCatalog();
    fn xmlLoadCatalog(filename: *const c_char) -> c_int;
    fn xmlCatalogResolvePublic(pub_id: *const u8) -> *mut u8;
    fn xmlCatalogResolveSystem(sys_id: *const u8) -> *mut u8;
    fn xmlCatalogResolve(pub_id: *const u8, sys_id: *const u8) -> *mut u8;
    fn xmlCatalogResolveURI(uri: *const u8) -> *mut u8;
    fn xmlCatalogDump(out: *mut FILE);
    fn xmlCatalogAdd(type_: *const u8, orig: *const u8, replace: *const u8) -> c_int;
    fn xmlCatalogRemove(value: *const u8) -> c_int;
    fn xmlCatalogConvert() -> c_int;
    fn xmlLoadSGMLSuperCatalog(filename: *const c_char) -> xmlCatalogPtr;
    fn xmlNewCatalog(sgml: c_int) -> xmlCatalogPtr;
    fn xmlACatalogAdd(catal: xmlCatalogPtr, type_: *const u8, orig: *const u8, replace: *const u8);
    fn xmlACatalogRemove(catal: xmlCatalogPtr, value: *const u8) -> c_int;
    fn xmlACatalogDump(catal: xmlCatalogPtr, out: *mut FILE);
    fn xmlCatalogIsEmpty(catal: xmlCatalogPtr) -> c_int;
    fn xmlParseURI(text: *const c_char) -> xmlURIPtr;
    fn xmlFreeURI(uri: xmlURIPtr);
    fn xmlCleanupParser();
    fn xmlMemoryDump();
    fn remove(path: *const c_char) -> c_int;
}

#[derive(Default)]
struct Config {
    shell: bool,
    sgml: bool,
    noout: bool,
    create: bool,
    add: bool,
    del: bool,
    convert: bool,
    no_super_update: bool,
    verbose: i32,
}

unsafe fn c_string_lossy(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

unsafe fn xml_string_lossy(ptr: *const u8) -> String {
    unsafe { c_string_lossy(ptr as *const c_char) }
}

unsafe fn free_xml_string(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(ptr as *mut c_void) };
    }
}

fn usage(name: &str) {
    println!(
        "Usage : {name} [options] catalogfile entities...\n\tParse the catalog file (void specification possibly expressed as \"\"\n\tappoints the default system one) and query it for the entities\n\t--sgml : handle SGML Super catalogs for --add and --del\n\t--shell : run a shell allowing interactive queries\n\t--create : create a new catalog\n\t--add 'type' 'orig' 'replace' : add an XML entry\n\t--add 'entry' : add an SGML entry\n\t--del 'values' : remove values\n\t--noout: avoid dumping the result on stdout\n\t         used with --add or --del, it saves the catalog changes\n\t         and with --sgml it automatically updates the super catalog\n\t--no-super-update: do not update the SGML super catalog\n\t-v --verbose : provide debug information"
    );
}

fn parse_args(args: &[std::ffi::OsString]) -> Result<(Config, usize), i32> {
    let mut cfg = Config::default();
    let mut i = 1usize;

    while i < args.len() {
        let arg = args[i].to_string_lossy();
        if arg == "-" || !arg.starts_with('-') {
            break;
        }
        match arg.as_ref() {
            "-verbose" | "-v" | "--verbose" => cfg.verbose += 1,
            "-noout" | "--noout" => cfg.noout = true,
            "-shell" | "--shell" => {
                cfg.shell = true;
                cfg.noout = true;
            }
            "-sgml" | "--sgml" => cfg.sgml = true,
            "-create" | "--create" => cfg.create = true,
            "-convert" | "--convert" => cfg.convert = true,
            "-no-super-update" | "--no-super-update" => cfg.no_super_update = true,
            "-add" | "--add" => {
                cfg.add = true;
                i += if cfg.sgml { 2 } else { 3 };
            }
            "-del" | "--del" => {
                cfg.del = true;
                i += 1;
            }
            _ => {
                eprintln!("Unknown option {arg}");
                usage(&args[0].to_string_lossy());
                return Err(1);
            }
        }
        i += 1;
    }

    Ok((cfg, i))
}

unsafe fn parse_shell_tokens(line: *const c_char) -> (String, Vec<String>) {
    let bytes = unsafe { CStr::from_ptr(line) }.to_bytes();
    let mut cur = 0usize;
    while cur < bytes.len() && (bytes[cur] == b' ' || bytes[cur] == b'\t') {
        cur += 1;
    }

    let mut command = Vec::new();
    while cur < bytes.len()
        && bytes[cur] != b' '
        && bytes[cur] != b'\t'
        && bytes[cur] != b'\n'
        && bytes[cur] != b'\r'
        && command.len() < MAX_COMMAND_SIZE - 1
    {
        command.push(bytes[cur]);
        cur += 1;
    }

    while cur < bytes.len() && (bytes[cur] == b' ' || bytes[cur] == b'\t') {
        cur += 1;
    }

    let mut arg = Vec::new();
    while cur < bytes.len()
        && bytes[cur] != b'\n'
        && bytes[cur] != b'\r'
        && arg.len() < MAX_ARG_SIZE - 1
    {
        arg.push(bytes[cur]);
        cur += 1;
    }

    let mut argv = Vec::new();
    let mut idx = 0usize;
    while idx < arg.len() {
        while idx < arg.len() && (arg[idx] == b' ' || arg[idx] == b'\t') {
            idx += 1;
        }
        if idx >= arg.len() {
            break;
        }
        let quote = if arg[idx] == b'\'' || arg[idx] == b'"' {
            let q = arg[idx];
            idx += 1;
            Some(q)
        } else {
            None
        };
        let start = idx;
        while idx < arg.len() {
            let ch = arg[idx];
            if let Some(q) = quote {
                if ch == q {
                    break;
                }
            } else if ch == b' ' || ch == b'\t' {
                break;
            }
            idx += 1;
        }
        argv.push(String::from_utf8_lossy(&arg[start..idx]).into_owned());
        if quote.is_some() && idx < arg.len() {
            idx += 1;
        }
    }

    (String::from_utf8_lossy(&command).into_owned(), argv)
}

unsafe fn print_catalog_answer(answer: *mut u8, missing: &str, output: *mut FILE) {
    if answer.is_null() {
        unsafe { write_file_str(output, missing) };
    } else {
        unsafe {
            write_file_str(output, &xml_string_lossy(answer));
            write_file_str(output, "\n");
            free_xml_string(answer);
        }
    }
}

unsafe fn run_shell(cfg: &mut Config) {
    let prompt = b"> \0";
    loop {
        let cmdline = unsafe { c_readline_with_prompt(prompt.as_ptr() as *const c_char) };
        if cmdline.is_null() {
            return;
        }
        let (command, argv) = unsafe { parse_shell_tokens(cmdline) };
        unsafe { free_c_ptr(cmdline as *mut c_void) };
        if command.is_empty() {
            continue;
        }
        let output = unsafe { output_or_stdout(stdout_handle()) };
        match command.as_str() {
            "exit" | "quit" | "bye" => break,
            "public" => {
                if argv.len() != 1 {
                    unsafe { write_file_str(output, "public requires 1 arguments\n") };
                } else if let Ok(public_id) = CString::new(argv[0].as_str()) {
                    let ans = unsafe { xmlCatalogResolvePublic(public_id.as_ptr() as *const u8) };
                    unsafe {
                        print_catalog_answer(
                            ans,
                            &format!("No entry for PUBLIC {}\n", argv[0]),
                            output,
                        );
                    }
                }
            }
            "system" => {
                if argv.len() != 1 {
                    unsafe { write_file_str(output, "system requires 1 arguments\n") };
                } else if let Ok(system_id) = CString::new(argv[0].as_str()) {
                    let ans = unsafe { xmlCatalogResolveSystem(system_id.as_ptr() as *const u8) };
                    unsafe {
                        print_catalog_answer(
                            ans,
                            &format!("No entry for SYSTEM {}\n", argv[0]),
                            output,
                        );
                    }
                }
            }
            "resolve" => {
                if argv.len() != 2 {
                    unsafe { write_file_str(output, "resolve requires 2 arguments\n") };
                } else if let (Ok(pub_id), Ok(sys_id)) = (
                    CString::new(argv[0].as_str()),
                    CString::new(argv[1].as_str()),
                ) {
                    let ans = unsafe {
                        xmlCatalogResolve(
                            pub_id.as_ptr() as *const u8,
                            sys_id.as_ptr() as *const u8,
                        )
                    };
                    unsafe {
                        print_catalog_answer(ans, "Resolver failed to find an answer\n", output);
                    }
                }
            }
            "add" => {
                if argv.len() != 2 && argv.len() != 3 {
                    unsafe { write_file_str(output, "add requires 2 or 3 arguments\n") };
                } else {
                    let type_ = CString::new(argv[0].as_str()).ok();
                    let orig = if argv.len() == 3 {
                        CString::new(argv[1].as_str()).ok()
                    } else {
                        None
                    };
                    let replace = CString::new(argv[argv.len() - 1].as_str()).ok();
                    if let (Some(type_), Some(replace)) = (type_, replace) {
                        let ret = unsafe {
                            xmlCatalogAdd(
                                type_.as_ptr() as *const u8,
                                orig.as_ref().map_or(null(), |s| s.as_ptr() as *const u8),
                                replace.as_ptr() as *const u8,
                            )
                        };
                        if ret != 0 {
                            unsafe { write_file_str(output, "add command failed\n") };
                        }
                    }
                }
            }
            "del" => {
                if argv.len() != 1 {
                    unsafe { write_file_str(output, "del requires 1\n") };
                } else if let Ok(value) = CString::new(argv[0].as_str()) {
                    if unsafe { xmlCatalogRemove(value.as_ptr() as *const u8) } <= 0 {
                        unsafe { write_file_str(output, "del command failed\n") };
                    }
                }
            }
            "dump" => {
                if !argv.is_empty() {
                    unsafe { write_file_str(output, "dump has no arguments\n") };
                } else {
                    unsafe { xmlCatalogDump(output) };
                }
            }
            "debug" => {
                if !argv.is_empty() {
                    unsafe { write_file_str(output, "debug has no arguments\n") };
                } else {
                    cfg.verbose += 1;
                    unsafe {
                        let _ = xmlCatalogSetDebug(cfg.verbose);
                    }
                }
            }
            "quiet" => {
                if !argv.is_empty() {
                    unsafe { write_file_str(output, "quiet has no arguments\n") };
                } else {
                    cfg.verbose = (cfg.verbose - 1).max(0);
                    unsafe {
                        let _ = xmlCatalogSetDebug(cfg.verbose);
                    }
                }
            }
            _ => unsafe {
                if command != "help" {
                    write_file_str(output, &format!("Unrecognized command {}\n", command));
                }
                write_file_str(output, "Commands available:\n");
                write_file_str(
                    output,
                    "\tpublic PublicID: make a PUBLIC identifier lookup\n",
                );
                write_file_str(
                    output,
                    "\tsystem SystemID: make a SYSTEM identifier lookup\n",
                );
                write_file_str(
                    output,
                    "\tresolve PublicID SystemID: do a full resolver lookup\n",
                );
                write_file_str(output, "\tadd 'type' 'orig' 'replace' : add an entry\n");
                write_file_str(output, "\tdel 'values' : remove values\n");
                write_file_str(output, "\tdump: print the current catalog state\n");
                write_file_str(output, "\tdebug: increase the verbosity level\n");
                write_file_str(output, "\tquiet: decrease the verbosity level\n");
                write_file_str(output, "\texit:  quit the shell\n");
            },
        }
    }
}

unsafe fn save_xml_catalog(filename: &str, noout: bool, exit_value: &mut i32) {
    if noout && !filename.is_empty() {
        if let Ok(filename_c) = CString::new(filename) {
            let out = unsafe { fopen_write(filename_c.as_ptr()) };
            if out.is_null() {
                eprintln!("could not open {} for saving", filename);
                *exit_value = 2;
            } else {
                unsafe {
                    xmlCatalogDump(out);
                    fclose_file(out);
                }
            }
        }
    } else {
        unsafe { xmlCatalogDump(stdout_handle()) };
    }
}

unsafe fn save_sgml_catalog(
    catal: xmlCatalogPtr,
    filename: &CStr,
    noout: bool,
    exit_value: &mut i32,
) {
    if !noout {
        unsafe { xmlACatalogDump(catal, stdout_handle()) };
        return;
    }
    if unsafe { xmlCatalogIsEmpty(catal) } != 0 {
        unsafe {
            let _ = remove(filename.as_ptr());
        }
        return;
    }
    let out = unsafe { fopen_write(filename.as_ptr()) };
    if out.is_null() {
        eprintln!("could not open {} for saving", filename.to_string_lossy());
        *exit_value = 2;
        return;
    }
    unsafe {
        xmlACatalogDump(catal, out);
        fclose_file(out);
    }
}

pub fn main() -> i32 {
    let args = collect_args();
    if args.len() <= 1 {
        usage(&args[0].to_string_lossy());
        return 1;
    }

    let (mut cfg, first_non_option) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(code) => return code,
    };

    unsafe {
        let _ = xmlCatalogSetDebug(cfg.verbose);
    }

    let mut filename: Option<String> = None;
    let mut first_entity = first_non_option;
    while first_entity < args.len() {
        let arg = args[first_entity].to_string_lossy().into_owned();
        if arg.is_empty() && filename.is_none() {
            unsafe { xmlInitializeCatalog() };
            filename = Some(arg);
        } else if filename.is_none() {
            if let Ok(catalog_path) = cstring_from_os(args[first_entity].as_os_str()) {
                let ret = unsafe { xmlLoadCatalog(catalog_path.as_ptr()) };
                if ret < 0 && cfg.create {
                    unsafe {
                        let _ = xmlCatalogAdd(
                            b"catalog\0".as_ptr(),
                            catalog_path.as_ptr() as *const u8,
                            null(),
                        );
                    }
                }
                filename = Some(arg);
            }
        } else {
            break;
        }
        first_entity += 1;
        break;
    }

    if cfg.convert {
        unsafe {
            let _ = xmlCatalogConvert();
        }
    }

    let mut exit_value = 0i32;

    if cfg.add || cfg.del {
        let mut i = 1usize;
        while i < args.len() {
            let arg = args[i].to_string_lossy();
            if arg == "-" {
                break;
            }
            if !arg.starts_with('-') {
                i += 1;
                continue;
            }
            match arg.as_ref() {
                "-add" | "--add" if cfg.sgml => {
                    if i + 2 >= args.len() {
                        exit_value = 1;
                        break;
                    }
                    let catalog_file = match cstring_from_os(args[i + 1].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let entry = match cstring_from_os(args[i + 2].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let mut catal = unsafe { xmlLoadSGMLSuperCatalog(catalog_file.as_ptr()) };
                    if catal.is_null() {
                        catal = unsafe { xmlNewCatalog(1) };
                    }
                    unsafe {
                        xmlACatalogAdd(
                            catal,
                            b"CATALOG\0".as_ptr(),
                            entry.as_ptr() as *const u8,
                            null(),
                        )
                    };

                    let mut super_catalog = null_mut();
                    if !cfg.no_super_update {
                        let default =
                            CString::new(XML_SGML_DEFAULT_CATALOG).expect("static string");
                        super_catalog = unsafe { xmlLoadSGMLSuperCatalog(default.as_ptr()) };
                        if super_catalog.is_null() {
                            super_catalog = unsafe { xmlNewCatalog(1) };
                        }
                        unsafe {
                            xmlACatalogAdd(
                                super_catalog,
                                b"CATALOG\0".as_ptr(),
                                catalog_file.as_ptr() as *const u8,
                                null(),
                            );
                        }
                        unsafe {
                            save_sgml_catalog(
                                super_catalog,
                                default.as_c_str(),
                                cfg.noout,
                                &mut exit_value,
                            );
                        }
                    }
                    unsafe {
                        save_sgml_catalog(
                            catal,
                            catalog_file.as_c_str(),
                            cfg.noout,
                            &mut exit_value,
                        );
                    }
                    i += 3;
                }
                "-del" | "--del" if cfg.sgml => {
                    if i + 2 >= args.len() {
                        eprintln!("No catalog entry specified to remove from");
                        usage(&args[0].to_string_lossy());
                        return 1;
                    }
                    let catalog_file = match cstring_from_os(args[i + 1].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let value = match cstring_from_os(args[i + 2].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let catal = unsafe { xmlLoadSGMLSuperCatalog(catalog_file.as_ptr()) };
                    if catal.is_null()
                        || unsafe { xmlACatalogRemove(catal, value.as_ptr() as *const u8) } < 0
                    {
                        eprintln!(
                            "Failed to remove entry from {}",
                            catalog_file.to_string_lossy()
                        );
                        exit_value = 1;
                    } else {
                        unsafe {
                            save_sgml_catalog(
                                catal,
                                catalog_file.as_c_str(),
                                cfg.noout,
                                &mut exit_value,
                            );
                        }
                    }
                    i += 3;
                }
                "-add" | "--add" => {
                    if i + 3 >= args.len() {
                        exit_value = 1;
                        break;
                    }
                    let type_ = match cstring_from_os(args[i + 1].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let orig = match cstring_from_os(args[i + 2].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let replace = match cstring_from_os(args[i + 3].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    let ret = unsafe {
                        xmlCatalogAdd(
                            type_.as_ptr() as *const u8,
                            if replace.to_bytes().is_empty() {
                                null()
                            } else {
                                orig.as_ptr() as *const u8
                            },
                            replace.as_ptr() as *const u8,
                        )
                    };
                    if ret != 0 {
                        println!("add command failed");
                        exit_value = 3;
                    }
                    i += 4;
                }
                "-del" | "--del" => {
                    if i + 1 >= args.len() {
                        eprintln!("No catalog entry specified to remove from");
                        usage(&args[0].to_string_lossy());
                        return 1;
                    }
                    let value = match cstring_from_os(args[i + 1].as_os_str()) {
                        Ok(path) => path,
                        Err(code) => return code,
                    };
                    if unsafe { xmlCatalogRemove(value.as_ptr() as *const u8) } < 0 {
                        eprintln!("Failed to remove entry {}", value.to_string_lossy());
                        exit_value = 1;
                    }
                    i += 2;
                }
                _ => i += 1,
            }
        }

        if !cfg.sgml && (cfg.add || cfg.del || cfg.create || cfg.convert) {
            unsafe {
                save_xml_catalog(
                    filename.as_deref().unwrap_or_default(),
                    cfg.noout,
                    &mut exit_value,
                );
            }
        }
    } else if cfg.shell {
        unsafe { run_shell(&mut cfg) };
    } else {
        let start = if filename.is_some() {
            first_entity
        } else {
            first_non_option
        };
        for arg in args.iter().skip(start) {
            let entity = match cstring_from_os(arg.as_os_str()) {
                Ok(entity) => entity,
                Err(code) => return code,
            };
            let uri = unsafe { xmlParseURI(entity.as_ptr()) };
            if uri.is_null() {
                let ans = unsafe { xmlCatalogResolvePublic(entity.as_ptr() as *const u8) };
                if ans.is_null() {
                    println!("No entry for PUBLIC {}", entity.to_string_lossy());
                    exit_value = 4;
                } else {
                    unsafe {
                        println!("{}", xml_string_lossy(ans));
                        free_xml_string(ans);
                    }
                }
            } else {
                unsafe { xmlFreeURI(uri) };
                let mut ans = unsafe { xmlCatalogResolveSystem(entity.as_ptr() as *const u8) };
                if ans.is_null() {
                    println!("No entry for SYSTEM {}", entity.to_string_lossy());
                    ans = unsafe { xmlCatalogResolveURI(entity.as_ptr() as *const u8) };
                    if ans.is_null() {
                        println!("No entry for URI {}", entity.to_string_lossy());
                        exit_value = 4;
                    } else {
                        unsafe {
                            println!("{}", xml_string_lossy(ans));
                            free_xml_string(ans);
                        }
                    }
                } else {
                    unsafe {
                        println!("{}", xml_string_lossy(ans));
                        free_xml_string(ans);
                    }
                }
            }
        }
    }

    unsafe {
        xmlCleanupParser();
        xmlMemoryDump();
    }
    exit_value
}
