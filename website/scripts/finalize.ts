import { cp, mkdir, readFile, readdir, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { emptyImplementorScripts, emptyRegistration } from './rustdoc';

const provenance = JSON.parse(await readFile('.generated/provenance.json', 'utf8'));
const rustdocDir = path.join('out/rustdoc', provenance.revision);
await rm(rustdocDir, { recursive: true, force: true });
await cp(provenance.rustdocInput, rustdocDir, { recursive: true });
const index = '<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>re-cordis exact Rustdoc</title></head><body><main><h1>re-cordis exact Rustdoc</h1><p>Current documentation for published experimental ' + provenance.version + ' packages. Release source: ' + provenance.releaseRevision + '. Build revision: ' + provenance.revision + '</p><ul>' + provenance.crates.map((crate: string) => '<li><a href="re_cordis_' + crate + '/index.html">re-cordis-' + crate + '</a></li>').join('') + '</ul></main></body></html>';
const generatedLanding = !(await Bun.file(path.join(provenance.rustdocInput, 'index.html')).exists());
if (generatedLanding) await writeFile(path.join(rustdocDir, 'index.html'), index);
const generatedEmptyImplementors = [];
for (const entry of emptyImplementorScripts) {
  if (await Bun.file(path.join(provenance.rustdocInput, entry.file)).exists()) continue;
  const basename = path.basename(entry.file);
  for await (const candidate of new Bun.Glob('**/' + basename).scan(path.join(provenance.rustdocInput, 'trait.impl'))) {
    throw new Error('An alternate emitted implementor registry exists; inspect before generating: ' + candidate);
  }
  const references = [];
  for (const source of entry.sources) {
    const html = await readFile(path.join(provenance.rustdocInput, source), 'utf8');
    const relative = path.posix.relative(path.posix.dirname(source), entry.file);
    if (!html.includes('src="' + relative + '"')) throw new Error('Expected missing implementor reference changed: ' + source);
    references.push({ source, sha256: createHash('sha256').update(html).digest('hex') });
  }
  const file = path.join(rustdocDir, entry.file);
  await mkdir(path.dirname(file), { recursive: true });
  await writeFile(file, emptyRegistration(entry.owner));
  generatedEmptyImplementors.push({ ...entry, references, reason: 'Compiler emitted a script reference but no out-of-crate implementor registry; use its empty registration protocol.' });
}
const rustdocFiles: Record<string, string> = {};
async function fingerprint(directory: string) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) await fingerprint(file);
    else if (entry.isFile()) rustdocFiles[path.relative(rustdocDir, file)] = createHash('sha256').update(await readFile(file)).digest('hex');
    else throw new Error(`Unexpected Rustdoc filesystem entry: ${file}`);
  }
}
await fingerprint(rustdocDir);
const emittedRustdocFiles: Record<string, string> = {};
for (const [file, expected] of Object.entries(rustdocFiles)) {
  if ((generatedLanding && file === 'index.html') || generatedEmptyImplementors.some((entry) => entry.file === file)) continue;
  const actual = createHash('sha256').update(await readFile(path.join(provenance.rustdocInput, file))).digest('hex');
  if (actual !== expected) throw new Error('Compiler-emitted Rustdoc bytes changed: ' + file);
  emittedRustdocFiles[file] = actual;
}

for (const source of Object.keys(provenance.sources)) {
  const digest = createHash('sha256').update(await readFile(path.join('..', source))).digest('hex');
  if (digest !== provenance.sources[source]) throw new Error(`Source changed during build: ${source}`);
}
await stat('out/404.html');
function publicFileHashes(files: Record<string, string>) {
  return Object.fromEntries(Object.entries(files).map(([file, digest]) => [createHash('sha256').update(file).digest('hex'), digest]));
}
const publicProvenance = { ...provenance };
delete publicProvenance.rustdocInput;
await writeFile('out/build-provenance.json', JSON.stringify({ ...publicProvenance, rustdocFiles: publicFileHashes(rustdocFiles), emittedRustdocFiles: publicFileHashes(emittedRustdocFiles), rustdocFileKeyFormat: 'sha256-relative-path', generatedLanding, generatedEmptyImplementors }, null, 2));
console.log(`Bundled exact Rustdoc (${Object.keys(rustdocFiles).length} files) at ${provenance.basePath}/rustdoc/${provenance.revision}/.`);
