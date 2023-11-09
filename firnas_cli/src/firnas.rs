use clap::{Subcommand, ValueEnum};
use firnas_compiler::compiler;
use firnas_interpreter::interpreter::Interpreter;
use firnas_tokenizer::tokenizer;
use firnas_vm::virtual_machine;
use std::fs;

#[derive(Subcommand)]
pub enum Firnas {
    /// Lunch Read-Eval-Print Loop
    Repl {
        /// Extend the language with Work-in-Progress features
        #[clap(short = 'X', value_parser, num_args = 1.., value_delimiter = ' ')]
        extentions: Vec<Extension>,
    },
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
    pub fn handle_repl(extentions: &[Extension]) -> anyhow::Result<()> {
        println!("==== Fernas repl ====");
        let extensions = firnas_ext::Extensions {
            lists: extentions.contains(&Extension::Lists),
            lambdas: extentions.contains(&Extension::Lambdas),
        };
        let mut interpreter = Interpreter::default();
        loop {
            print!("> ");
            let line: String = text_io::read!("{}\n");
            if line.is_empty() {
                break;
            }
            let tokens = tokenizer::scan_tokens(line).unwrap();
            let stmts = firnas_interpreter::parser::parse(extensions, tokens).unwrap();
            interpreter.interpret(&stmts).unwrap();
        }
        Ok(())
    }

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