use std::path::Path;

use ckb_rpc::module::*;
use serde_json::Value;

pub const OPENRPC_DIR: &str = "./docs/ckb_rpc_openrpc/";
pub const OPENRPC_DIR_REPO: &str = "https://github.com/nervosnetwork/ckb-rpc-resources";

macro_rules! generate_docs {
    ($($func:ident),* $(,)?) => {
        [
            $(
                (format!("{}.json", stringify!($func)), $func()),
            )*
        ]
    };
}

pub(crate) fn all_rpc_docs() -> Vec<(String, Value)> {
    generate_docs!(
        alert_rpc_doc,
        net_rpc_doc,
        subscription_rpc_doc,
        debug_rpc_doc,
        chain_rpc_doc,
        miner_rpc_doc,
        pool_rpc_doc,
        stats_rpc_doc,
        integration_test_rpc_doc,
        indexer_rpc_doc,
        experiment_rpc_doc,
    )
    .into()
}

pub(crate) fn run_command(prog: &str, args: &[&str], dir: Option<&str>) -> Option<String> {
    std::process::Command::new(prog)
        .args(args)
        .current_dir(dir.unwrap_or("."))
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|r| {
            String::from_utf8(r.stdout)
                .ok()
                .map(|s| s.trim().to_string())
        })
}

pub(crate) fn get_version() -> String {
    let version = run_command("cargo", &["pkgid"], None)
        .unwrap()
        .split('#')
        .nth(1)
        .unwrap_or("0.0.0")
        .to_owned();
    let stripped = version.split('@').nth(1).unwrap_or(&version).trim();
    stripped.to_string()
}

pub(crate) fn get_current_git_branch() -> String {
    run_command("git", &["rev-parse", "--abbrev-ref", "HEAD"], None)
        .unwrap_or_else(|| "unknown".to_string())
}

pub(crate) fn checkout_openrpc_branch(version: &str) {
    let git_dir = format!("{}/.git", OPENRPC_DIR);
    if !Path::new(&git_dir).exists() {
        let _ = run_command("rm", &["-r", OPENRPC_DIR], None);
        let _ = run_command("git", &["clone", OPENRPC_DIR_REPO, "ckb-rpc-repo"], None);
        let _ = run_command("mv", &["ckb-rpc-repo", OPENRPC_DIR], None);
    }
    eprintln!("checkout version: {}", version);
    let dir = Some(OPENRPC_DIR);
    let res = run_command("git", &["checkout", version], dir);
    if res.is_none() {
        run_command("git", &["checkout", "-b", version], dir);
    }
}

pub(crate) fn is_git_repo_dirty() -> bool {
    let res = run_command("git", &["status", "--porcelain"], Some(OPENRPC_DIR));
    res.map(|s| !s.is_empty()).unwrap_or(false)
}

pub(crate) fn get_git_remote_url() -> String {
    run_command(
        "git",
        &["config", "--get", "remote.origin.url"],
        Some(OPENRPC_DIR),
    )
    .map_or("".to_string(), |s| s.trim().to_string())
}
