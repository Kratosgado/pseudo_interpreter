-- Standalone Neovim setup (no LazyVim / no plugin manager required).
-- Source from init.lua:
--   dofile(vim.fn.expand("~/projects/rust/frontier_model_1/editors/neovim/pseudo_lsp.lua"))
--
-- Optional: override binary path before sourcing:
--   vim.g.pseudo_lsp_cmd = "/absolute/path/to/pseudo_lsp"
--
-- Requires: nvim-lspconfig

local GITHUB_REPO = "Kratosgado/pseudo_interpreter"

-- Add this directory to runtimepath for syntax/ and ftdetect/
local here = vim.fn.fnamemodify( debug.getinfo( 1, "S" ).source:sub( 2 ), ":p:h" )
if not vim.tbl_contains( vim.opt.runtimepath:get(), here ) then
    vim.opt.runtimepath:append( here )
end
vim.filetype.add( { extension = { ps = "pseudo", pseudo = "pseudo" } } )

-- ── binary management ────────────────────────────────────────────────────────

local function platform_asset()
    if vim.fn.has( "win32" ) == 1 then return "pseudo_lsp-windows-x64.exe" end
    local uname = vim.uv.os_uname()
    local sys   = uname.sysname:lower()
    local mach  = uname.machine:lower()
    if sys == "linux"  and mach == "x86_64" then return "pseudo_lsp-linux-x64"    end
    if sys == "darwin" and mach == "arm64"  then return "pseudo_lsp-darwin-arm64" end
    if sys == "darwin" and mach == "x86_64" then return "pseudo_lsp-darwin-x64"   end
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

local function install_server( callback )
    local asset = platform_asset()
    if not asset then
        vim.notify( "[pseudo_lsp] No pre-built binary for this platform. "
            .. "Build from source: cargo install --git https://github.com/"
            .. GITHUB_REPO .. " pseudo_lsp", vim.log.levels.ERROR )
        return
    end
    local bin = default_bin()
    local url = "https://github.com/" .. GITHUB_REPO
              .. "/releases/latest/download/" .. asset
    vim.fn.mkdir( vim.fn.fnamemodify( bin, ":h" ), "p" )
    vim.notify( "[pseudo_lsp] Downloading language server...", vim.log.levels.INFO )
    local cmd = vim.fn.has( "win32" ) == 1
        and { "powershell", "-Command",
              string.format( 'Invoke-WebRequest -Uri "%s" -OutFile "%s"', url, bin ) }
        or  { "curl", "-fsSL", "-o", bin, url }
    vim.system( cmd, {}, vim.schedule_wrap( function( result )
        if result.code ~= 0 then
            vim.notify( "[pseudo_lsp] Download failed: " .. ( result.stderr or "" ),
                vim.log.levels.ERROR )
            return
        end
        if vim.fn.has( "win32" ) == 0 then vim.system( { "chmod", "+x", bin } ) end
        vim.notify( "[pseudo_lsp] Installed. Re-open the file or run :LspStart pseudo_lsp.",
            vim.log.levels.INFO )
        if callback then callback( bin ) end
    end ) )
end

-- ── LSP setup ────────────────────────────────────────────────────────────────

local lspconfig = require( "lspconfig" )
local configs   = require( "lspconfig.configs" )

if not configs.pseudo_lsp then
    configs.pseudo_lsp = {
        default_config = {
            cmd            = { "pseudo_lsp" },
            filetypes      = { "pseudo" },
            root_dir       = lspconfig.util.root_pattern( ".git", "*.ps", "*.pseudo" ),
            single_file_support = true,
            on_new_config  = function( config )
                local bin = get_bin()
                if bin then
                    config.cmd = { bin }
                else
                    install_server( function()
                        vim.notify( "[pseudo_lsp] Run :LspStart pseudo_lsp to start.",
                            vim.log.levels.INFO )
                    end )
                    config.cmd = nil
                end
            end,
        },
    }
end

local ok, cmp_lsp   = pcall( require, "cmp_nvim_lsp" )
local capabilities  = ok
    and cmp_lsp.default_capabilities()
    or  vim.lsp.protocol.make_client_capabilities()

lspconfig.pseudo_lsp.setup( {
    capabilities = capabilities,
    on_attach = function( _, bufnr )
        local o = { buffer = bufnr, silent = true }
        vim.keymap.set( "n", "K",          vim.lsp.buf.hover,         o )
        vim.keymap.set( "n", "<leader>rn", vim.lsp.buf.rename,        o )
        vim.keymap.set( "n", "<leader>ca", vim.lsp.buf.code_action,   o )
        vim.keymap.set( "n", "[d",         vim.diagnostic.goto_prev,  o )
        vim.keymap.set( "n", "]d",         vim.diagnostic.goto_next,  o )
        vim.keymap.set( "n", "<leader>d",  vim.diagnostic.open_float, o )
    end,
} )
