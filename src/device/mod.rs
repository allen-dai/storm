use std::sync::{Arc, Mutex};

use crate::{
    codegen::linearizer::{Linearizer, LinearizerOptions},
    dtype::Dtype,
    ops::LazyOp,
    renderer::cstyle::{uops_to_cstyle, LanguageOpts, Renderer},
    shape::symbolic::NodeOp,
};

lazy_static::lazy_static! {
    pub static ref DEVICE: Arc<dyn Device> = Arc::new(opencl::CLDevice::new());
    pub static ref PENDING_COPY: Mutex<PendingCopy> = Mutex::new(PendingCopy::default());
}

#[derive(Default, Debug)]
pub struct PendingCopy(Vec<Vec<u8>>);

unsafe impl Send for PendingCopy {}
unsafe impl Sync for PendingCopy {}

pub mod opencl;

pub mod prelude {
    pub use super::opencl::{CLBuffer, CLDevice, CLProgram};
    pub use super::{DEVICE, PENDING_COPY};
}

pub trait Device: Send + Sync + core::fmt::Debug {
    fn alloc(&self, size: usize, dtype: Dtype) -> Arc<dyn Buffer>;
    fn build(&self, name: &str, program: &str) -> Arc<dyn Program>;
    fn copyout(&self, src: &dyn Buffer, dst: *mut u8);
    fn copyin(&self, src: Vec<u8>, dst: &dyn Buffer);
    fn synchronize(&self);
    fn linearizer_opts(&self) -> LinearizerOptions {
        LinearizerOptions::default()
    }
    fn renderer(&self) -> Arc<dyn Renderer>;
    fn get_lin(&self, ast: LazyOp) -> Linearizer {
        Linearizer::new(ast, Some(self.linearizer_opts()))
    }
    fn render(&self, mut lin: Linearizer) -> (String, String) {
        lin.linearize();
        let prg = uops_to_cstyle(self.renderer(), &lin.name, &lin.uops);
        (lin.name, prg)
    }
}

pub trait Program: core::fmt::Debug {
    fn run(
        &self,
        bufs: &[Arc<dyn Buffer>],
        global_size: &[usize],
        local_size: Option<&[usize]>,
        args: &[isize],
        extra: &[String],
    );
}

pub trait Buffer: core::fmt::Debug {
    fn ptr(&self) -> *mut core::ffi::c_void;
    fn dtype(&self) -> Dtype;
    fn size(&self) -> usize;
    fn to_cpu(&self) -> Vec<u8>;
    fn from_cpu(&mut self, data: Vec<u8>);
}
