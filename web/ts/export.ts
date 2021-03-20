import fs from "fs";
import path from "path";
import assert from 'assert';

const ROOT_DIR = '.';

const DEST_DIR = 'dist-web';

function rmdirSync(dirName: string) {
  if (!fs.existsSync(dirName)) {
    return;
  }

  for (let filename of fs.readdirSync(dirName)) {
    const absPath = path.join(dirName, filename);
    if (fs.statSync(absPath).isDirectory()) {
      rmdirSync(absPath);
    } else {
      fs.unlinkSync(absPath);
    }
  }

  fs.rmdirSync(dirName);
}

function copyFilesInDirSync(source: string, dest: string) {
  for (let filename of fs.readdirSync(source)) {
    const content = fs.readFileSync(path.join(source, filename));
    fs.writeFileSync(path.join(dest, filename), content);
  }
}

function copyFileSync(src: string, dest: string) {
  console.log(`Copying "${src}" to "${dest}".`);
  const content = fs.readFileSync(src);
  fs.writeFileSync(dest, content);
}

function copyFilesToDirSync(srcDir: string, files: string[], destDir: string) {
  for (let file of files) {
    copyFileSync(path.join(srcDir, file), path.join(destDir, file));
  }
}

function mkdirSync(dirname: string) {
  console.log(`Creating "${dirname}".`);
  fs.mkdirSync(dirname);
}

function main() {
  assert(fs.existsSync(path.join(ROOT_DIR, 'package.json')));

  rmdirSync(DEST_DIR);
  mkdirSync(DEST_DIR);

  copyFilesToDirSync(ROOT_DIR, ['index.html', 'level.bmp'], DEST_DIR);

  mkdirSync(path.join(DEST_DIR, 'web'));
  for (let relDir of [
    path.join('web', 'pkg'),
    path.join('web', 'ts'),
  ]) {
    const srcDir = path.join(ROOT_DIR, relDir);
    const destDir = path.join(DEST_DIR, relDir);
    mkdirSync(destDir);
    copyFilesInDirSync(srcDir, destDir);
  }
}

main();
