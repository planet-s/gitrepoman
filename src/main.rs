extern crate gitlab;
extern crate github_rs;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate rayon;
extern crate serde_json;
extern crate clap;

mod action;
mod actions;
mod config;
mod gitlab_impl;
mod github_impl;

use self::action::Action;
use self::actions::GitAction;
use self::config::*;
use self::github_impl::GitHub;
use std::process::exit;
use gitlab::Gitlab;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("gitrepoman")
        .version("0.1")
        .about("Manages git repositories across GitHub organizations or GitLab instances")
        .author("Michael Aaron Murphy")
        .arg(Arg::with_name("ssh")
            .long("--ssh")
            .short("s"))
        .arg(Arg::with_name("force")
            .long("force")
            .short("f"))
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .takes_value(true))
        .subcommand(SubCommand::with_name("gitlab")
            .arg(Arg::with_name("DOMAIN").required(true))
            .arg(Arg::with_name("NAMESPACE").required(true))
            .arg(Arg::with_name("ACTION").required(true)))
        .subcommand(SubCommand::with_name("github")
            .arg(Arg::with_name("DOMAIN").required(true))
            .arg(Arg::with_name("ACTION").required(true)))
        .get_matches();

    let config_path: &str = matches.value_of("config").unwrap_or("secret.toml");

    let config = match Config::new(config_path) {
        Ok(config) => config,
        Err(why) => {
            eprintln!("failed to get config: {}", why);
            exit(1);
        }
    };

    let flags = if matches.occurrences_of("ssh") > 0 { 0b01 } else { 0b00 }
        + if matches.occurrences_of("force") > 0 { 0b10 } else { 0b00 };

    let (source, org, action, ns) = if let Some(matches) = matches.subcommand_matches("gitlab") {
        (
            GitService::GitLab,
            matches.value_of("DOMAIN").unwrap(),
            matches.value_of("ACTION").unwrap(),
            matches.value_of("NAMESPACE").unwrap_or("")
        )
    } else if let Some(matches) = matches.subcommand_matches("github") {
        (
            GitService::GitHub,
            matches.value_of("DOMAIN").unwrap(),
            matches.value_of("ACTION").unwrap(),
            ""
        )
    } else {
        eprintln!("no subcommand provided");
        exit(1);
    };

    macro_rules! client {
        ($name:tt, $token:expr) => {{
            let token = match $token {
                Some(token) => token,
                None => {
                    eprintln!("no {} token provided", stringify!($name));
                    exit(1);
                }
            };

            match $name::new(org.to_owned(), token) {
                Ok(client) => Box::new(client),
                Err(why) => {
                    eprintln!("unable to authenticate client: {}", why);
                    exit(1);
                }
            }


        }};
    }

    let authenticated: Box<GitAction> = match source {
        GitService::GitHub => client!(GitHub, config.github),
        GitService::GitLab => client!(Gitlab, config.gitlab),
    };

    match Action::from(action) {
        Ok(Action::List) => authenticated.list(ns),
        Ok(Action::Clone) => authenticated.clone(flags, ns),
        Ok(Action::Pull) => authenticated.pull(flags, ns),
        Ok(Action::Checkout) => authenticated.checkout(flags, ns),
        Ok(Action::MirrorPull) => authenticated.mirror_pull(flags, ns),
        Ok(Action::MirrorPush) => authenticated.mirror_push(flags, ns),
        Err(cmd) => {
            eprintln!("{} is not a valid command", cmd);
            exit(1);
        }
    }
}
