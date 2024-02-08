import re
import sys
import zipfile

apk_file_path = sys.argv[1]
version_pattern = r"d+\.d+\.(\d+)"

with zipfile.ZipFile(apk_file_path, 'r') as zip_ref:
  myHealthBundlePath = "assets/myhealth.bundle"
  with zip_ref.open(myHealthBundlePath) as file:
    file_contents = file.read().decode("utf-8")
    match = re.search(version_pattern, file_contents)
    if match:
      version = match.group(1)
      print("MyHealth SIT Version: " + version)
    else:
      print("Version not found")

 