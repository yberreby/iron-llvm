// Copyright 2015 Jauhien Piatlicki.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// LLVM Execution Engine
// LLVM-C header ExecutionEngine.h

use std;
use std::ffi::CString;

use libc::{c_char, c_uint};

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;

use ::{LLVMRef, LLVMRefCtor};
use core;
use execution_engine::GenericValue;

pub struct ExecutionEngine {
    ee: LLVMExecutionEngineRef
}

impl ExecutionEngine {
    pub fn new(mut module: core::Module) -> Result<ExecutionEngine, String> {
        let mut ee = 0 as LLVMExecutionEngineRef;
        let mut error = 0 as *mut c_char;
        unsafe {
            module.unown();
            if LLVMCreateExecutionEngineForModule(&mut ee, module.to_ref(), &mut error) > 0 {
                let cstr_buf = std::ffi::CStr::from_ptr(error);
                let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
                LLVMDisposeMessage(error);
                Err(result)
            } else {
                Ok(ExecutionEngine {ee:ee})
            }
        }
    }

    pub fn new_interpreter(mut module: core::Module) -> Result<ExecutionEngine, String> {
        let mut ee = 0 as LLVMExecutionEngineRef;
        let mut error = 0 as *mut c_char;
        unsafe {
            module.unown();
            if LLVMCreateInterpreterForModule(&mut ee, module.to_ref(), &mut error) > 0 {
                let cstr_buf = std::ffi::CStr::from_ptr(error);
                let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
                LLVMDisposeMessage(error);
                Err(result)
            } else {
                Ok(ExecutionEngine {ee:ee})
            }
        }
    }

    pub fn new_jit_compiler(mut module: core::Module, opt_level: u32) -> Result<ExecutionEngine, String> {
        let mut ee = 0 as LLVMExecutionEngineRef;
        let mut error = 0 as *mut c_char;
        unsafe {
            module.unown();
            if LLVMCreateJITCompilerForModule(&mut ee, module.to_ref(), opt_level, &mut error) > 0 {
                let cstr_buf = std::ffi::CStr::from_ptr(error);
                let result = String::from_utf8_lossy(cstr_buf.to_bytes()).into_owned();
                LLVMDisposeMessage(error);
                Err(result)
            } else {
                Ok(ExecutionEngine {ee:ee})
            }
        }
    }

    pub fn run_static_constructors(&mut self) {
        unsafe {
            LLVMRunStaticConstructors(self.to_ref())
        }
    }

    pub fn run_static_destructors(&mut self) {
        unsafe {
            LLVMRunStaticDestructors(self.to_ref())
        }
    }

    pub fn run_function<T:core::Function>(&mut self, f: &T, args: &mut [LLVMGenericValueRef]) -> GenericValue {
        unsafe {
            GenericValue::from_ref(LLVMRunFunction(self.to_ref(), f.to_ref(), args.len() as c_uint, args.as_mut_ptr()))
        }
    }

    pub fn add_module(&self, mut module: core::Module) {
        unsafe {
            LLVMAddModule(self.to_ref(), module.to_ref());
            module.unown();
        }
    }

    pub fn get_function_address(&self, name: &str) -> u64 {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMGetFunctionAddress(self.to_ref(), name.as_ptr() as *const c_char)
        }
    }
}

impl LLVMRef<LLVMExecutionEngineRef> for ExecutionEngine {
    fn to_ref(&self) -> LLVMExecutionEngineRef {
        self.ee
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.to_ref())
        }
    }
}

pub fn link_in_mcjit() {
    unsafe {
        LLVMLinkInMCJIT();
    }
}

pub fn link_in_interpreter() {
    unsafe {
        LLVMLinkInInterpreter();
    }
}