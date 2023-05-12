2023-05-12T18:28:27.319 helix_vcs [ERROR] file is untracked
2023-05-12T18:28:27.319 helix_vcs [ERROR] failed to open diff base for /Users/zisulin/prx/shuttle/services/shuttle-crond/src/lib.rs
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "[ERROR rust_analyzer::main_loop] FetchWorkspaceError:\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "rust-analyzer failed to load workspace: Failed to read Cargo metadata from Cargo.toml file /Users/zisulin/prx/shuttle/Cargo.toml, Some(Version { major: 1, minor: 69, patch: 0 }): Failed to run `cd \"/Users/zisulin/prx/shuttle\" && \"cargo\" \"metadata\" \"--format-version\" \"1\" \"--manifest-path\" \"/Users/zisulin/prx/shuttle/Cargo.toml\" \"--filter-platform\" \"aarch64-apple-darwin\"`: `cargo metadata` exited with an error: error: failed to run `rustc` to learn about target-specific information\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "Caused by:\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "  process didn't exit successfully: `/Users/zisulin/.cargo/bin/sccache rustc - --crate-name ___ --print=file-names --target aarch64-apple-darwin --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 2)\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "  --- stderr\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "  sccache: error: Timed out waiting for server startup\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.499 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.500 helix_term::application [WARN] unhandled window/showMessage: ShowMessageParams { typ: Error, message: "Failed to load workspaces." }
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "[ERROR rust_analyzer::main_loop] FetchWorkspaceError:\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "rust-analyzer failed to load workspace: Failed to read Cargo metadata from Cargo.toml file /Users/zisulin/prx/shuttle/Cargo.toml, Some(Version { major: 1, minor: 69, patch: 0 }): Failed to run `cd \"/Users/zisulin/prx/shuttle\" && \"cargo\" \"metadata\" \"--format-version\" \"1\" \"--manifest-path\" \"/Users/zisulin/prx/shuttle/Cargo.toml\" \"--filter-platform\" \"aarch64-apple-darwin\"`: `cargo metadata` exited with an error: error: failed to run `rustc` to learn about target-specific information\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "Caused by:\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "  process didn't exit successfully: `/Users/zisulin/.cargo/bin/sccache rustc - --crate-name ___ --print=file-names --target aarch64-apple-darwin --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=split-debuginfo --print=crate-name --print=cfg` (exit status: 2)\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "  --- stderr\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "  sccache: error: Timed out waiting for server startup\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.670 helix_lsp::transport [ERROR] err <- "\n"
2023-05-12T18:28:27.671 helix_term::application [WARN] unhandled window/showMessage: ShowMessageParams { typ: Error, message: "Failed to load workspaces." }
