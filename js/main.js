
const fs = require('fs');
const path = require('path');
const JSZip = require('jszip');

const apkFilePath = process.argv[2];
if (!apkFilePath) {
  console.error("Usage: node main.js <path to apk file>");
  process.exit();
}
const versionPattern = /2\.3\.(\d+)/;
fs.readFile(apkFilePath, (err, data) => {
  if (err) {
    console.error(err);
    return;
  }

  JSZip.loadAsync(data).then((zip) => {
    const myHealthBundlePath = "assets/myhealth.bundle";
    const file = zip.file(myHealthBundlePath);
    if (file) {
      file.async("string").then((fileContents) => {
        const match = fileContents.match(versionPattern);
        if (match) {
          const version = match[1];
          console.log("MyHealth SIT Version: " + version);
        } else {
          console.log("Version not found");
        }
      });
    } else {
      console.log("File not found: " + myHealthBundlePath);
    }
  }).catch((err) => {
    console.error(err);
  });
});
