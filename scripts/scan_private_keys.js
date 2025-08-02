const fs = require('fs');
const path = require('path');
const process = require('process');

// Simple argument parsing (can use commander for better handling)
const args = process.argv.slice(2);
const verbose = args.includes('--verbose');
const ignoreDirs = new Set(['.git', 'node_modules']); // Add from --ignore if needed

function isTextFile(filepath) {
	try {
		fs.readFileSync(filepath, 'utf8');
		return true;
	} catch (err) {
		return false; // Assume binary if decode fails
	}
}

function checkForPrivateKeys(rootDir, ignoreDirs) {
	const suspiciousFiles = [];
	const namePattern = /(id_(rsa|dsa|ecdsa)|.*\.(pem|key|ppk))$/i;
	const contentPattern = /-----BEGIN\s+(RSA|EC|DSA|PRIVATE)\s+KEY-----/;

	function walkDir(dir) {
		const files = fs.readdirSync(dir);
		files.forEach((file) => {
			const filepath = path.join(dir, file);
			const stat = fs.statSync(filepath);

			if (stat.isDirectory()) {
				if (!ignoreDirs.has(file)) {
					walkDir(filepath);
				}
			} else {
				if (namePattern.test(file)) {
					suspiciousFiles.push(filepath);
				} else if (isTextFile(filepath)) {
					const content = fs.readFileSync(filepath, 'utf8');
					if (contentPattern.test(content)) {
						suspiciousFiles.push(filepath);
					}
				}
			}
		});
	}

	walkDir(rootDir);
	return suspiciousFiles;
}

function main() {
	const rootDir = process.cwd(); // Or from args
	const suspicious = checkForPrivateKeys(rootDir, ignoreDirs);

	if (suspicious.length > 0) {
		console.log('Suspicious files found:');
		suspicious.forEach((file) => console.log(file));
		process.exit(1);
	} else {
		console.log('No private keys detected.');
		process.exit(0);
	}
}

main();
