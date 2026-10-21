import * as https from "https";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";
import {
    workspace,
    ExtensionContext,
    window,
    commands,
    ProgressLocation,
} from "vscode";
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    type Executable,
} from "vscode-languageclient/node";
import * as vscode from "vscode";

const REPO = "Kratosgado/pseudo_interpreter";

let client: LanguageClient;

export async function activate( context: ExtensionContext ) {
    context.subscriptions.push(
        commands.registerCommand( "pseudoRunner.installServer", () =>
            installServer( context, true )
        )
    );

    const serverPath = await resolveServer( context );
    if ( !serverPath ) return;

    startClient( context, serverPath );
}

export function deactivate(): Thenable<void> | undefined {
    return client?.stop();
}

// ── server resolution ────────────────────────────────────────────────────────

async function resolveServer( context: ExtensionContext ): Promise<string | null> {
    // 1. User-configured path
    const configured = workspace
        .getConfiguration( "pseudoLsp" )
        .get<string>( "serverPath" );
    if ( configured && configured.length > 0 ) {
        if ( !fs.existsSync( configured ) ) {
            window.showErrorMessage(
                `pseudo_lsp: configured path not found: ${configured}`
            );
            return null;
        }
        return configured;
    }

    // 2. Cached binary from a previous install
    const cached = cachedBinPath( context );
    if ( fs.existsSync( cached ) ) return cached;

    // 3. First-time: download automatically
    return installServer( context, false );
}

async function installServer(
    context: ExtensionContext,
    force: boolean
): Promise<string | null> {
    const asset = platformAsset();
    if ( !asset ) {
        window.showErrorMessage(
            `pseudo_lsp: no pre-built binary for ${process.platform}/${process.arch}. ` +
                "Build from source: cargo install --git " +
                `https://github.com/${REPO} pseudo_lsp`
        );
        return null;
    }

    const dest = cachedBinPath( context );
    if ( !force && fs.existsSync( dest ) ) return dest;

    const url = `https://github.com/${REPO}/releases/latest/download/${asset}`;

    return window.withProgress(
        {
            location: ProgressLocation.Notification,
            title: "Pseudo LSP: downloading language server...",
            cancellable: false,
        },
        async () => {
            try {
                fs.mkdirSync( path.dirname( dest ), { recursive: true } );
                await downloadFile( url, dest );
                if ( process.platform !== "win32" ) fs.chmodSync( dest, 0o755 );

                if ( force ) {
                    const choice = await window.showInformationMessage(
                        "Pseudo LSP: language server installed.",
                        "Reload Window"
                    );
                    if ( choice === "Reload Window" )
                        commands.executeCommand( "workbench.action.reloadWindow" );
                }
                return dest;
            } catch ( err ) {
                window.showErrorMessage(
                    `Pseudo LSP: download failed — ${err}. ` +
                        'Run "Pseudo LSP: Install Server" to retry.'
                );
                return null;
            }
        }
    );
}

// ── client startup ───────────────────────────────────────────────────────────

function startClient( context: ExtensionContext, serverPath: string ) {
    const run: Executable = {
        command: serverPath,
        transport: TransportKind.stdio,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "pseudo" }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher( "**/*.{ps,pseudo}" ),
        },
    };

    client = new LanguageClient(
        "pseudoLsp",
        "Pseudo LSP",
        { run, debug: run } as ServerOptions,
        clientOptions
    );

    client.start();

    context.subscriptions.push(
        commands.registerCommand( "pseudoRunner.runPseudo", () =>
            runPseudo( serverPath )
        )
    );
}

// ── helpers ──────────────────────────────────────────────────────────────────

function cachedBinPath( context: ExtensionContext ): string {
    const ext = process.platform === "win32" ? ".exe" : "";
    return path.join(
        context.globalStorageUri.fsPath,
        "bin",
        `pseudo_lsp${ext}`
    );
}

function platformAsset(): string | null {
    const p = process.platform;
    const a = process.arch;
    if ( p === "linux" && a === "x64" ) return "pseudo_lsp-linux-x64";
    if ( p === "darwin" && a === "arm64" ) return "pseudo_lsp-darwin-arm64";
    if ( p === "darwin" && a === "x64" ) return "pseudo_lsp-darwin-x64";
    if ( p === "win32" && a === "x64" ) return "pseudo_lsp-windows-x64.exe";
    return null;
}

function downloadFile( url: string, dest: string ): Promise<void> {
    return new Promise( ( resolve, reject ) => {
        const follow = ( href: string ) => {
            https
                .get( href, { headers: { "User-Agent": "pseudo-runner-vscode" } }, ( res ) => {
                    const code = res.statusCode ?? 0;
                    if ( [ 301, 302, 307, 308 ].includes( code ) && res.headers.location ) {
                        follow( res.headers.location );
                        return;
                    }
                    if ( code !== 200 ) {
                        reject( new Error( `HTTP ${code} from ${href}` ) );
                        return;
                    }
                    const tmp = dest + ".tmp";
                    const file = fs.createWriteStream( tmp );
                    res.pipe( file );
                    file.on( "finish", () =>
                        file.close( ( err ) => {
                            if ( err ) return reject( err );
                            fs.renameSync( tmp, dest );
                            resolve();
                        } )
                    );
                    file.on( "error", ( err ) => {
                        fs.rmSync( tmp, { force: true } );
                        reject( err );
                    } );
                } )
                .on( "error", reject );
        };
        follow( url );
    } );
}

async function runPseudo( serverPath: string ) {
    const editor = vscode.window.activeTextEditor;
    if ( !editor || editor.document.languageId !== "pseudo" ) return;

    const fileName = editor.document.fileName;
    const binDir = path.dirname( serverPath );
    const ext = os.platform() === "win32" ? ".exe" : "";
    const interpreterPath = path.join( binDir, `pseudo_interpreter${ext}` );

    const terminal =
        vscode.window.terminals.find( ( t ) => t.name === "Pseudo Runner" ) ??
        vscode.window.createTerminal( "Pseudo Runner" );

    terminal.show();
    terminal.sendText( `"${interpreterPath}" "${fileName}"` );
}
