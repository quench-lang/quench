import { createWriteStream, promises as fs } from 'fs';
import * as https from 'https';
import * as os from 'os';
import * as path from 'path';
import * as process from 'process';

import * as vscode from 'vscode';

import * as lsp from 'vscode-languageclient/node';

let client: lsp.LanguageClient;

function defaultServerPath(version: string): string {
  return path.join(os.homedir(), '.quench', version, 'bin', 'quench-lsp');
}

function serverPath(version: string): string {
  const conf = vscode.workspace.getConfiguration('quench').get('server.path');
  if (typeof conf === 'string') {
    return conf;
  } else {
    return defaultServerPath(version);
  }
}

function translatePlatform(platform: string): string {
  if (platform === 'darwin') {
    return 'macos';
  } else if (platform === 'win32') {
    return 'windows';
  } else {
    return platform;
  }
}

function start(command: string) {
  client = new lsp.LanguageClient(
    'quench',
    'Quench',
    { command },
    { documentSelector: [{ scheme: 'file', language: 'quench' }] },
  );
  client.start();
}

export async function activate(context: vscode.ExtensionContext) {
  const manifest = path.join(context.extensionPath, 'package.json');
  const { version } = JSON.parse(await fs.readFile(manifest, 'utf8'));
  let server = serverPath(version);
  const exists = await fs.stat(server).then(() => true, () => false);
  if (exists) {
    start(server);
  } else {
    server = defaultServerPath(version);
    const dir = path.dirname(server);
    // mkdir succeeds if dir already exists, since we set recursive to true
    await fs.mkdir(dir, { 'recursive': true });

    const download = 'Download';
    const userResponse = await vscode.window.showInformationMessage(
      'Quench language server is not installed.',
      download,
    );
    if (userResponse === download) {
      const platform = translatePlatform(process.platform);
      const ext = platform === 'windows' ? '.exe' : '';
      const url = `https://github.com/quench-lang/quench/releases/download/v${version}/quench-lsp-${platform}${ext}`;
      // https://stackoverflow.com/a/11944984
      https.get(url, response => {
        const { statusCode } = response;
        if (statusCode === 200) {
          // https://stackoverflow.com/a/39850268
          const file = createWriteStream(server, { mode: 0o755 });
          response.pipe(file).on('finish', () => {
            start(server);
          });
        } else {
          vscode.window.showErrorMessage(
            `Quench language server failed to download with status code ${statusCode}.`,
          );
        }
      });
    } else {
      vscode.window.showErrorMessage('Quench language server not downloaded.');
    }
  }
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
