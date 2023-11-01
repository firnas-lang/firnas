use clap::Parser;
use clap::Subcommand;
use firnas_compiler::compiler;
use firnas_ext;
use firnas_interpreter::interpreter::Interpreter;
use firnas_tokenizer::scanner;
use firnas_vm::virtual_machine;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Client,
}

#[derive(Subcommand)]
pub enum Client {
    /// Lunch Read-Eval-Print Loop
    Repl,
    /// Compile a file
    Compile {
        /// Path to file
        path: String,
    },
}

impl Client {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Repl => Client::handle_repl(),
            Self::Compile { path } => Client::handle_file(path.to_string()),
        }
    }

    fn handle_repl() -> anyhow::Result<()> {
        println!("==== Fernas repl ====");
        let extensions = firnas_ext::Extensions {
            lists: true,
            lambdas: false,
        };
        let mut interpreter = Interpreter::default();
        loop {
            print!("> ");
            let line: String = text_io::read!("{}\n");
            if line.is_empty() {
                break;
            }
            let tokens = scanner::scan_tokens(line).unwrap();
            let stmts = firnas_interpreter::parser::parse(extensions, tokens).unwrap();
            interpreter.interpret(&stmts).unwrap();
        }
        Ok(())
    }

    fn handle_file(path: String) -> anyhow::Result<()> {
        let content = fs::read_to_string(path)?;
        let extensions = firnas_ext::Extensions {
            lists: true,
            lambdas: false,
        };

        let func_or_err = compiler::Compiler::compile(content, extensions);

        let _ = func_or_err.map(|f| {
            let mut interpreter = virtual_machine::VirtualMachine::default();
            interpreter.interpret(f).unwrap();
        });
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
