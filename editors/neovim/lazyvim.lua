-- Drop this file at:  ~/.config/nvim/lua/plugins/pseudo.lua
--
-- Update the `repo` path below to wherever you cloned this repo.
-- Set vim.g.pseudo_lsp_cmd to override the binary path (optional).

local repo = vim.fn.expand( "~/projects/rust/frontier_model_1" )
local GITHUB_REPO = "Kratosgado/pseudo_interpreter"

-- ── binary management ────────────────────────────────────────────────────────

local function platform_asset()
    if vim.fn.has( "win32" ) == 1 then
        return "pseudo_lsp-windows-x64.exe"
    end
    local uname = vim.uv.os_uname()
    local sysname = uname.sysname:lower()
    local machine = uname.machine:lower()
    if sysname == "linux" and machine == "x86_64" then
        return "pseudo_lsp-linux-x64"
    elseif sysname == "darwin" and machine == "arm64" then
        return "pseudo_lsp-darwin-arm64"
    elseif sysname == "darwin" and machine == "x86_64" then
        return "pseudo_lsp-darwin-x64"
    end
    return nil
end

local function default_bin()
    local ext = vim.fn.has( "win32" ) == 1 and ".exe" or ""
    return vim.fn.stdpath( "data" ) .. "/pseudo_lsp/pseudo_lsp" .. ext
end

local function get_bin()
    if vim.g.pseudo_lsp_cmd and vim.g.pseudo_lsp_cmd ~= "" then
        return vim.g.pseudo_lsp_cmd
    end
    if vim.fn.executable( "pseudo_lsp" ) == 1 then
        return vim.fn.exepath( "pseudo_lsp" )
    end
    local bin = default_bin()
    if vim.fn.filereadable( bin ) == 1 then return bin end
    return nil
end

local function install_server( opts )
    opts = opts or {}
    local asset = platform_asset()
    if not asset then
        vim.notify( "[pseudo_lsp] No pre-built binary for this platform.\n"
            .. "Build from source: cargo install --git https://github.com/"
            .. GITHUB_REPO .. " pseudo_lsp", vim.log.levels.ERROR )
        return
    end

    local bin  = default_bin()
    local dir  = vim.fn.fnamemodify( bin, ":h" )
    local url  = "https://github.com/" .. GITHUB_REPO
               .. "/releases/latest/download/" .. asset

    vim.fn.mkdir( dir, "p" )
    vim.notify( "[pseudo_lsp] Downloading language server...", vim.log.levels.INFO )

    local cmd = vim.fn.has( "win32" ) == 1
        and { "powershell", "-Command",
              string.format( 'Invoke-WebRequest -Uri "%s" -OutFile "%s"', url, bin ) }
        or  { "curl", "-fsSL", "-o", bin, url }

    vim.system( cmd, {}, vim.schedule_wrap( function( result )
        if result.code ~= 0 then
            vim.notify( "[pseudo_lsp] Download failed:\n" .. ( result.stderr or "" ),
                vim.log.levels.ERROR )
            return
        end
        if vim.fn.has( "win32" ) == 0 then
            vim.system( { "chmod", "+x", bin } )
        end
        vim.notify( "[pseudo_lsp] Language server installed. Re-open a .ps file to start.",
            vim.log.levels.INFO )
        if opts.callback then opts.callback( bin ) end
    end ) )
end

-- ── plugin spec ──────────────────────────────────────────────────────────────

return {
    -- Load syntax highlighting and filetype detection from the repo
    {
        dir    = repo .. "/editors/neovim",
        name   = "pseudo.nvim",
        lazy   = false,
        priority = 100,
    },

    -- LSP via nvim-lspconfig
    {
        "neovim/nvim-lspconfig",
        opts = function( _, opts )
            local configs = require( "lspconfig.configs" )

            if not configs.pseudo_lsp then
                configs.pseudo_lsp = {
                    default_config = {
                        cmd            = { "pseudo_lsp" },  -- overridden in on_new_config
                        filetypes      = { "pseudo" },
                        root_dir       = require( "lspconfig" ).util.root_pattern(
                                             ".git", "*.ps", "*.pseudo" ),
                        single_file_support = true,
                        settings       = {},
                        on_new_config  = function( config )
                            local bin = get_bin()
                            if bin then
                                config.cmd = { bin }
                            else
                                -- Binary missing: download then prompt
                                install_server( {
                                    callback = function()
                                        vim.notify(
                                            "[pseudo_lsp] Re-open the file or run :LspStart pseudo_lsp",
                                            vim.log.levels.INFO )
                                    end,
                                } )
                                config.cmd = nil  -- prevents lspconfig from starting
                            end
                        end,
                    },
                }
            end

            opts.servers             = opts.servers or {}
            opts.servers.pseudo_lsp  = {}
        end,
    },
}
