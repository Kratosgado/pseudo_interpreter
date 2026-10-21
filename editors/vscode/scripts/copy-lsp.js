#!/usr/bin/env node
// Builds pseudo_lsp (debug mode, fast) and copies the binary to bin/pseudo_lsp.
// Works regardless of CARGO_TARGET_DIR setting.

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");

const manifestPath = path.resolve(__dirname, "../../../Cargo.toml");

let output;
try {
    output = execSync(
        `cargo build -p pseudo_lsp --manifest-path "${manifestPath}" --message-format=json`,
        { encoding: "utf8", stdio: ["pipe", "pipe", "inherit"] }
    );
} catch (err) {
    process.stderr.write("cargo build failed\n");
    process.exit(1);
}

for (const line of output.split("\n")) {
    if (!line.trim()) continue;
    let msg;
    try { msg = JSON.parse(line); } catch { continue; }
    if (
        msg.reason === "compiler-artifact" &&
        msg.executable &&
        msg.executable.includes("pseudo_lsp") &&
        !msg.executable.includes("build-script")
    ) {
        fs.mkdirSync(path.join(__dirname, "../bin"), { recursive: true });
        const dest = path.join(__dirname, "../bin", "pseudo_lsp");
        fs.copyFileSync(msg.executable, dest);
        fs.chmodSync(dest, 0o755);
        process.stdout.write(`Copied ${msg.executable} -> bin/pseudo_lsp\n`);
        process.exit(0);
    }
}

process.stderr.write("Error: could not locate pseudo_lsp binary in cargo output\n");
process.exit(1);
