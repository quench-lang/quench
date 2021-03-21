import * as assert from 'assert';
import * as path from 'path';
import * as vscode from 'vscode';

async function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function activate(docUri: vscode.Uri) {
  const ext = vscode.extensions.getExtension('quench.quench')!;
  await ext.activate();
  try {
    const doc = await vscode.workspace.openTextDocument(docUri);
    return await vscode.window.showTextDocument(doc);
  } catch (e) {
    console.error(e);
  }
}

function getExamplePath(p: string) {
  // very precarious
  return path.resolve(__dirname, '../../../../../examples', p);
}

function getExampleUri(p: string) {
  return vscode.Uri.file(getExamplePath(p));
}

function toRange(sLine: number, sChar: number, eLine: number, eChar: number) {
  const start = new vscode.Position(sLine, sChar);
  const end = new vscode.Position(eLine, eChar);
  return new vscode.Range(start, end);
}

suite('Quench', function () {
  test('correctly reports syntax error(s) for "Hello, world!"', async function () {
    this.timeout(10000); // longboi
    const docUri = getExampleUri('hello.qn');
    const editor = (await activate(docUri))!;
    await sleep(1000); // wait half a sec for the lang server to start
    assert.deepStrictEqual(vscode.languages.getDiagnostics(docUri), []);
    await editor.edit(editBuilder => {
      // mangle
      editBuilder.replace(toRange(2, 0, 3, 0), 'print("Hello,"" world!")');
    });
    await sleep(1000); // wait half a sec for the lang server to re-parse
    {
      const diagnostics = vscode.languages.getDiagnostics(docUri);
      assert.strictEqual(diagnostics.length, 2);
      const expected = [
        new vscode.Diagnostic(toRange(2, 6, 2, 14), 'syntax (ERROR (string))'),
        new vscode.Diagnostic(toRange(2, 24, 2, 24), 'syntax (MISSING ";")'),
      ];
      expected.forEach((expected, i) => {
        const actual = diagnostics[i];
        assert.strictEqual(actual.message, expected.message);
        assert.deepStrictEqual(actual.range, expected.range);
        assert.strictEqual(actual.severity, expected.severity);
      });
    }
    await editor.edit(editBuilder => {
      // unmangle
      editBuilder.replace(toRange(2, 0, 3, 0), 'print("Hello, world!");');
    });
    await sleep(1000); // wait half a sec for the lang server re-parse again
    assert.deepStrictEqual(vscode.languages.getDiagnostics(docUri), []);
  });
});
