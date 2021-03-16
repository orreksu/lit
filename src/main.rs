
use std::process::{Command, Stdio};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Oleksandr Litus <oleks.litus@gmail.com")]
struct Opts {
    
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version= "1.3", author = "Oleksandr Litus <oleks.litus@gmail.com>")]
    Init(Init),
    Update(Update),
    Sync(Sync),
    Tree(Tree)
}

#[derive(Clap)]
struct Init {

    #[clap(default_value = "My new lit project")]
    name: String
}

impl Init {
    fn cmd(&self) {
        Command::new("echo")
                .arg(format!("\"# {}\"", &self.name))
                .arg(">>")
                .arg("README.md")
                .output()
                .expect("failed to git add");

        Command::new("git")
                .arg("init")
                .output()
                .expect("failed to git add");

        Update { 
            msg: "First update".to_string() 
        }.cmd();

        Command::new("git")
                .arg("branch")
                .arg("-M")
                .arg("main")
                .output()
                .expect("failed to git add");
    }
}


#[derive(Clap)]
struct Update {
    msg: String
}

impl Update {
    fn cmd(&self) {
        Command::new("git")
                .arg("add")
                .arg("--all")
                .output()
                .expect("failed to git add");

        Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(&self.msg)
                .output()
                .expect("failed to git commit");
    }
}

#[derive(Clap)]
struct Sync;

impl Sync {
    fn cmd(&self) {
        Command::new("git")
                .arg("pull")
                .output()
                .expect("failed to git pull");

        Command::new("git")
                .arg("push")
                .output()
                .expect("failed to git push");
    }
}


#[derive(Clap)]
struct Tree;

impl Tree {
    fn cmd(&self) {
        let output = Command::new("git")
                .arg("log")
                .arg("--all")
                .arg("--graph")
                .arg("--oneline")
                .arg("--decorate")
                .stdout(Stdio::piped())
                .output()
                .expect("failed to git log");

        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout);
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    
    match opts.subcmd {
        SubCommand::Init(i)   => i.cmd(),
        SubCommand::Update(u) => u.cmd(),
        SubCommand::Sync(s)   => s.cmd(),
        SubCommand::Tree(t)   => t.cmd(),
    }
}
