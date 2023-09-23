use crate::grammar::*;

use llvm_sys::core::*;
use llvm_sys::prelude::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish().to_string()
}

pub struct Compiler {
    ast: File,
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl Compiler {
    pub fn compile(filepath: &str, ast: File) {
        let compiler;
        let modulename = hash(filepath);
        let fileinputname = filepath.split(".").nth(0).unwrap();
        let fileoutputname = fileinputname.to_owned() + ".bdsm";
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(
                modulename.as_str().as_ptr() as *const i8,
                context,
            );
            let builder = LLVMCreateBuilderInContext(context);
            LLVMSetSourceFileName(
                module,
                fileinputname.as_ptr() as *const i8,
                fileinputname.len() as usize,
            );
            compiler = Compiler {
                ast,
                context,
                module,
                builder,
            };
        }
        for stmt in &compiler.ast.statements {
            compiler.compile_statement(stmt);
        }
        unsafe {
            LLVMPrintModuleToFile(
                compiler.module,
                fileoutputname.as_str().as_ptr() as *const i8,
                std::ptr::null_mut::<*mut i8>(),
            );
        }
    }

    fn compile_statement(&self, stmt: &Statement) {
        match &stmt.r#fn {
            Some(r#fn) => self.compile_fn(&r#fn),
            None => {}
        }
    }

    fn compile_fn(&self, r#fn: &FunctionDef) {
        let name = r#fn.name.as_str().as_ptr() as *const i8;
        let fn_type = unsafe {
            let mut param_types = Vec::new();
            for param in &r#fn.param_list.params {
                match param.typ.typename.as_str() {
                    "int" => param_types.push(LLVMInt32TypeInContext(self.context)),
                    _ => {}
                }
            }
            LLVMFunctionType(
                LLVMInt32TypeInContext(self.context),
                param_types.as_mut_ptr(),
                param_types.len() as u32,
                0,
            )
        };
        let fn_value = unsafe { LLVMAddFunction(self.module, name, fn_type) };
        let entry = unsafe { LLVMAppendBasicBlockInContext(self.context, fn_value, name) };
        unsafe { LLVMPositionBuilderAtEnd(self.builder, entry) };
        for (i, param) in r#fn.param_list.params.iter().enumerate() {
            let name = param.name.as_str().as_ptr() as *const i8;
            let value = unsafe { LLVMGetParam(fn_value, i as u32) };
            unsafe { LLVMSetValueName2(value, name, param.name.as_str().len() as usize) };
        }
    }
}
