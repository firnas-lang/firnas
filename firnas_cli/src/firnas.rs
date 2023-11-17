use clap::Subcommand;
use clap::ValueEnum;
use firnas_compiler::compiler;
use firnas_vm::virtual_machine;
use std::fs;

#[derive(Subcommand)]
pub enum Firnas {
    /// Compile a file
    Compile {
        /// Path to file
        path: String,
        /// Extend the language with Work-in-Progress features
        #[clap(short = 'X', value_parser, num_args = 1.., value_delimiter = ' ')]
        extentions: Vec<Extension>,
    },
}

impl Firnas {
    pub fn handle_file(path: String, extentions: &[Extension]) -> anyhow::Result<()> {
        let content = fs::read_to_string(path)?;
        let extensions = firnas_ext::Extensions {
            lists: extentions.contains(&Extension::Lists),
            lambdas: extentions.contains(&Extension::Lambdas),
        };

        let func_or_err = compiler::Compiler::compile(content, extensions);

        let _ = func_or_err.map(|f| {
            let mut interpreter = virtual_machine::VirtualMachine::default();
            interpreter.interpret(f).unwrap();
        });
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Extension {
    /// Adds lists
    Lists,
    /// Adds lambda functions
    Lambdas,
}
