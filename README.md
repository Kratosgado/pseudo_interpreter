# Pseudo Interpreter

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A pseudocode interpreter written in Rust, with a language server (LSP) for editor support.

## Language features

- Variables, expressions, arrays
- `if / elseif / else / endif`
- `while / endwhile` and `for / to / step / endfor` loops
- Functions with parameters and recursion
- `print` / `input` I/O
- `break` / `continue`

## Editor support

Both editors automatically download the correct `pseudo_lsp` binary for your platform on first use — no manual setup required.

| Feature | VSCode | Neovim |
|---|---|---|
| Syntax highlighting | TextMate grammar | Vim syntax file |
| Diagnostics | real parser errors, exact line | real parser errors, exact line |
| Completion | keywords | keywords |
| Hover docs | keywords | keywords |

---

## VSCode

Install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=KratosGado.pseudo-runner):

```
ext install KratosGado.pseudo-runner
```

Or search **"Pseudo Runner"** in the Extensions panel.

Open any `.ps` or `.pseudo` file. The language server downloads automatically on first use (~5 MB).

> **Manually update the language server:** open the Command Palette (`Ctrl+Shift+P`) and run
> `Pseudo: Install / Update Language Server`.

---

## Neovim (LazyVim)

**Prerequisites:** [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig), `curl` on PATH.

1. Copy the plugin spec to your LazyVim plugins directory:
   ```bash
   cp editors/neovim/lazyvim.lua ~/.config/nvim/lua/plugins/pseudo.lua
   ```

2. Open the file and update the `repo` path at the top:
   ```lua
   local repo = vim.fn.expand( "~/projects/rust/frontier_model_1" )
   ```

3. Restart Neovim. Open a `.ps` or `.pseudo` file — the language server is downloaded automatically on first use.

> **Override the binary path** (if you built it yourself):
> Add `vim.g.pseudo_lsp_cmd = "/path/to/pseudo_lsp"` to `~/.config/nvim/lua/config/options.lua`.

---

## Neovim (standalone, no LazyVim)

```lua
-- in init.lua
dofile(vim.fn.expand("~/projects/rust/frontier_model_1/editors/neovim/pseudo_lsp.lua"))
```

Syntax highlighting, filetype detection, and the LSP are all set up by that one call. The binary downloads automatically the first time you open a `.ps` file.

---

## CLI

```bash
pseudo_interpreter <file.ps>
```

Download the interpreter binary from [Releases](https://github.com/Kratosgado/pseudo_interpreter/releases), or build from source:

```bash
cargo build --release
./target/release/pseudo_interpreter <file.ps>
```

---

## Releasing a new version (maintainers)

1. Push a version tag — this triggers the full release pipeline:
   ```bash
   git tag v1.2.3
   git push origin v1.2.3
   ```

2. The workflow automatically:
   - Builds `pseudo_lsp` for Linux x64, macOS x64, macOS arm64, Windows x64
   - Creates a GitHub Release with all binaries attached
   - Publishes the updated VSCode extension to the Marketplace

**Required secrets:** `VSCE_PAT` (VS Code Marketplace personal access token).

---

## Contact

- Email: [mbeahessilfieprince@gmail.com](mailto:mbeahessilfieprince@gmail.com)
- GitHub: [Kratosgado](https://github.com/Kratosgado)
