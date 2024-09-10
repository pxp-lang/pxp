const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

/** @type {LanguageClient} */
let client;

/** @param {vscode.ExtensionContext} context */
function activate(context) {
	console.log("PLS (PHP Language Server) activating!");

	let serverModule = "/Users/ryan/Projects/Pxp/pxp/target/debug/pls";
	let debugOptions = { execArgv: ["--nolazy", "--inspect=6009"] };

	let serverOptions = {
		run: { command: serverModule, transport: TransportKind.stdio },
		debug: {
			command: serverModule,
			transport: TransportKind.stdio,
			options: debugOptions,
		},
	};

	let clientOptions = {
		documentSelector: [{ scheme: "file", language: "php" }],
	};

	client = new LanguageClient(
		"pls",
		"PLS (PHP Language Server)",
		serverOptions,
		clientOptions
	);

	let disposable = vscode.commands.registerCommand('vscode-pls.hello', function () {
		vscode.window.showInformationMessage('Hello from PLS (PHP Language Server)!');
	});

	context.subscriptions.push(disposable);

	return client.start();
}

function deactivate() {
	if (!client || !client.needsStop) {
		return undefined;
	}

	return client.stop();
}

module.exports = {
	activate,
	deactivate
}
