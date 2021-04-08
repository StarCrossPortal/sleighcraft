//
//  Copyright 2021 StarCrossTech
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use filetime::FileTime;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DECOMPILER_SOURCE_BASE_CXX: &[&'static str] = &[
    "space.cc",
    "float.cc",
    "address.cc",
    "pcoderaw.cc",
    "translate.cc",
    "opcodes.cc",
    "globalcontext.cc",
    "capability.cc",
    "architecture.cc",
    "options.cc",
    "graph.cc",
    "cover.cc",
    "block.cc",
    "cast.cc",
    "typeop.cc",
    "database.cc",
    "cpool.cc",
    "comment.cc",
    "fspec.cc",
    "action.cc",
    "loadimage.cc",
    "varnode.cc",
    "op.cc",
    "type.cc",
    "variable.cc",
    "varmap.cc",
    "jumptable.cc",
    "emulate.cc",
    "emulateutil.cc",
    "flow.cc",
    "userop.cc",
    "funcdata.cc",
    "funcdata_block.cc",
    "funcdata_varnode.cc",
    "funcdata_op.cc",
    "pcodeinject.cc",
    "heritage.cc",
    "prefersplit.cc",
    "rangeutil.cc",
    "ruleaction.cc",
    "subflow.cc",
    "blockaction.cc",
    "merge.cc",
    "double.cc",
    "coreaction.cc",
    "condexe.cc",
    "override.cc",
    "dynamic.cc",
    "crc32.cc",
    "prettyprint.cc",
    "printlanguage.cc",
    "printc.cc",
    "printjava.cc",
    "memstate.cc",
    "opbehavior.cc",
    "paramid.cc",
    "transform.cc",
    "stringmanage.cc",
    "string_ghidra.cc",
    "ghidra_arch.cc",
    "typegrp_ghidra.cc",
    "cpool_ghidra.cc",
    "loadimage_ghidra.cc",
    "inject_ghidra.cc",
    "database_ghidra.cc",
    "inject_sleigh.cc",
    "ghidra_translate.cc",
    "ghidra_context.cc",
    "comment_ghidra.cc",
    "sleigh_arch.cc",
    "sleigh.cc",
    "filemanage.cc",
    "semantics.cc",
    "slghsymbol.cc",
    "context.cc",
    "sleighbase.cc",
    "slghpatexpress.cc",
    "slghpattern.cc",
    "pcodecompile.cc",
];

/*
const DECOMPILER_SOURCE_BASE_YACC: [&'static str; 1] = [
    "xml.y"
];

const SLEIGH_COMPILER_SOURCE_CXX: [&'static str; 1] = [
    "slghparse.y"
];

const SLEIGH_COMPILER_SOURCE_FLEX: [&'static str; 1] = [
    "slghscan.l"
];
*/

const DECOMPILER_SOURCE_SLEIGH_YACC: &[&'static str] = &["pcodeparse.y", "grammar.y", "xml.y"];

const PROXIES: &[&'static str] = &[
    "address_proxy.cc",
    "addrspace_proxy.cc",
    "cover_proxy.cc",
    "funcdata_proxy.cc",
    "loadimage_proxy.cc",
    "opbehavior_proxy.cc",
    "opcode_proxy.cc",
    "opcodes_proxy.cc",
    "typeop_proxy.cc",
    "block_proxy.cc",
    "varnode_proxy.cc",
    "varnodedata_proxy.cc",
    "variable_proxy.cc",
];

struct CompileOptions {
    sources: Vec<PathBuf>,
    objects: Vec<PathBuf>,
}

fn need_recompile(source: &Path) -> bool {
    let outdir = env::var("OUT_DIR").unwrap();

    let path = Path::new(&outdir).join(source);
    let mut path = PathBuf::from(path);
    path.set_extension("o");
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return true,
    };
    let object_mtime = FileTime::from_last_modification_time(&metadata);

    let metadata = fs::metadata(source).expect(&format!("source code {:?} not found", source));
    let source_mtime = FileTime::from_last_modification_time(&metadata);

    if source_mtime > object_mtime {
        true
    } else {
        false
    }
}

fn obj_path_from_src_path(src_path: &Path) -> PathBuf {
    let outdir = env::var("OUT_DIR").unwrap();
    let mut path = PathBuf::from(Path::new(&outdir).join(src_path));
    path.set_extension("o");
    path
}

fn prepare() -> CompileOptions {
    let mut objects = vec![];
    let mut sources = vec![];

    for src in DECOMPILER_SOURCE_BASE_CXX.iter() {
        let path = Path::new("src").join("cpp").join(src);
        if need_recompile(&path) {
            sources.push(path);
        } else {
            objects.push(obj_path_from_src_path(&path));
        }
    }

    for src in DECOMPILER_SOURCE_SLEIGH_YACC.iter() {
        let name = src.split(".").next().unwrap();
        let path = Path::new("src")
            .join("cpp")
            .join("gen")
            .join("bison")
            .join(&format!("{}.cpp", name));

        if need_recompile(&path) {
            sources.push(path);
        } else {
            objects.push(obj_path_from_src_path(&path));
        }
    }

    for src in PROXIES.iter() {
        let path = Path::new("src")
            .join("cpp")
            .join("bridge")
            .join("proxies")
            .join(src);
        if need_recompile(&path) {
            sources.push(path);
        } else {
            objects.push(obj_path_from_src_path(&path));
        }
    }

    CompileOptions { objects, sources }
}

fn main() {
    let compile_opts = prepare();
    let sleigh_src_file = Path::new("src").join("sleigh.rs");

    let mut target = cxx_build::bridge(sleigh_src_file);

    for obj in compile_opts.objects.iter() {
        target.object(obj);
    }
    let disasm_src_path= Path::new("src").join("cpp").join("bridge").join("disasm.cpp");
    let src_cpp = Path::new("src").join("cpp");
    let src_cpp_gen_bison = Path::new("src").join("cpp").join("gen").join("bison");
    let src_cpp_gen_flex = Path::new("src").join("cpp").join("gen").join("flex");
    #[cfg(target_os = "windows")]
    target.define("_WINDOWS", "1"); // This is assumed by ghidra, but not defined by msvc, strange.
    target
        .cpp(true)
        .warnings(false)
        .file(disasm_src_path)
        .files(compile_opts.sources)
        .flag_if_supported("-std=c++14")
        .include(src_cpp)
        .include(src_cpp_gen_bison)
        .include(src_cpp_gen_flex)
        .compile("sleigh");
}
