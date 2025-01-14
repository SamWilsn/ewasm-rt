use crate::execute::Execute;
use crate::resolver::RuntimeModuleImportResolver;
use wasmi::{ImportsBuilder, MemoryRef, Module, ModuleInstance};

#[derive(Clone)]
pub struct Runtime<'a> {
    pub(crate) code: &'a [u8],
    pub(crate) data: &'a [u8],
    pub(crate) pre_root: [u8; 32],
    pub(crate) post_root: [u8; 32],
    pub(crate) memory: Option<MemoryRef>,
}

impl<'a> Runtime<'a> {
    pub fn new(code: &'a [u8], data: &'a [u8], pre_root: [u8; 32]) -> Runtime<'a> {
        Runtime {
            code,
            data,
            pre_root,
            post_root: [0u8; 32],
            memory: None,
        }
    }
}

impl<'a> Execute<'a> for Runtime<'a> {
    fn execute(&'a mut self) -> [u8; 32] {
        let module = Module::from_buffer(self.code).expect("Module loading to succeed");
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("env", &RuntimeModuleImportResolver);

        let instance = ModuleInstance::new(&module, &imports)
            .expect("Module instantation expected to succeed")
            .assert_no_start();

        self.memory = Some(
            instance
                .export_by_name("memory")
                .expect("Module expected to have 'memory' export")
                .as_memory()
                .cloned()
                .expect("'memory' export should be a memory"),
        );

        instance
            .invoke_export("main", &[], self)
            .expect("Executed 'main'");

        self.post_root
    }
}
