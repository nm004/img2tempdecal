import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const WEB_DIR = path.resolve(__dirname, "..");
const RS_LIB_DIR = path.resolve(WEB_DIR, "rs_lib");
const OUTPUT_FILE = path.join(__dirname, "../src/lib/assets/licenses");

console.log("Resolving Cargo dependencies...");
let cargoPackages = [];
try {
  const metadataStr = execSync("cargo metadata --format-version 1", {
    cwd: RS_LIB_DIR,
    encoding: "utf-8",
  });
  const metadata = JSON.parse(metadataStr);
  cargoPackages = metadata.packages || [];
} catch (e) {
  console.error("Failed to run cargo metadata:", e.message);
}

console.log(
  `Found ${cargoPackages.length} Cargo packages (including workspace and dependencies).`,
);

console.log("Scanning npm dependencies...");
const nodeModulesDir = path.join(WEB_DIR, "node_modules");
const npmLicenses = [];
const visitedPaths = new Set();

function scanNodeModules(dir) {
  if (!fs.existsSync(dir)) return;
  const files = fs.readdirSync(dir, { withFileTypes: true });
  for (const file of files) {
    const folderPath = path.join(dir, file.name);

    // Resolve if it's a directory or a symlink to a directory
    let isDir = false;
    try {
      isDir =
        file.isDirectory() ||
        (file.isSymbolicLink() && fs.statSync(folderPath).isDirectory());
    } catch (e) {
      continue;
    }

    if (isDir) {
      if (file.name.startsWith(".")) continue;
      if (file.name === "node_modules") {
        scanNodeModules(folderPath);
        continue;
      }
      if (file.name.startsWith("@")) {
        // Scoped packages
        const subfiles = fs.readdirSync(folderPath, { withFileTypes: true });
        for (const subfile of subfiles) {
          const subfolderPath = path.join(folderPath, subfile.name);
          let isSubDir = false;
          try {
            isSubDir =
              subfile.isDirectory() ||
              (subfile.isSymbolicLink() &&
                fs.statSync(subfolderPath).isDirectory());
          } catch (e) {
            continue;
          }

          if (isSubDir) {
            processPackage(
              subfolderPath,
              `@${file.name.substring(1)}/${subfile.name}`,
            );
          }
        }
      } else {
        processPackage(folderPath, file.name);
      }
    }
  }
}

function processPackage(packagePath, packageName) {
  let realPath;
  try {
    realPath = fs.realpathSync(packagePath);
  } catch (e) {
    return;
  }

  // Prevent infinite loops and duplicate scans due to symlinks
  if (visitedPaths.has(realPath)) return;
  visitedPaths.add(realPath);

  const pkgJsonPath = path.join(realPath, "package.json");
  if (!fs.existsSync(pkgJsonPath)) {
    // Not a package directory, but let's check recursively just in case
    scanNodeModules(realPath);
    return;
  }

  let version = "unknown";
  try {
    const pkg = JSON.parse(fs.readFileSync(pkgJsonPath, "utf-8"));
    version = pkg.version || "unknown";
  } catch (e) {}

  // Find license files in packagePath (using realPath)
  const files = fs.readdirSync(realPath);
  for (const file of files) {
    const lower = file.toLowerCase();
    if (
      lower.startsWith("license") ||
      lower.startsWith("licence") ||
      lower.startsWith("copying")
    ) {
      const licensePath = path.join(realPath, file);
      try {
        if (fs.statSync(licensePath).isFile()) {
          npmLicenses.push({
            name: packageName,
            version: version,
            path: licensePath,
          });
        }
      } catch (e) {}
    }
  }

  // Also scan nested node_modules
  const nestedNodeModules = path.join(realPath, "node_modules");
  if (fs.existsSync(nestedNodeModules)) {
    scanNodeModules(nestedNodeModules);
  }
}

scanNodeModules(nodeModulesDir);
console.log(`Found ${npmLicenses.length} npm license files.`);

// Concatenate all
let outputContent = `========================================================================
npm Dependencies Licenses
========================================================================

`;

// Sort npm licenses for consistent ordering
npmLicenses.sort((a, b) => a.name.localeCompare(b.name));

for (const lic of npmLicenses) {
  try {
    const content = fs.readFileSync(lic.path, "utf-8");
    outputContent += `========================================================================
Dependency: npm - ${lic.name} (${lic.version})
File: ${path.basename(lic.path)}
========================================================================
${content}

`;
  } catch (e) {
    console.error(`Failed to read npm license for ${lic.name}:`, e.message);
  }
}

outputContent += `
========================================================================
Cargo Dependencies Licenses
========================================================================

`;

// Sort cargo packages for consistent ordering
cargoPackages.sort((a, b) => a.name.localeCompare(b.name));

for (const pkg of cargoPackages) {
  // Skip our own workspace members (path dependencies without external registry source)
  if (!pkg.source) {
    continue;
  }

  const pkgDir = path.dirname(pkg.manifest_path);
  if (!fs.existsSync(pkgDir)) continue;

  const files = fs.readdirSync(pkgDir);
  let licenseFound = false;
  for (const file of files) {
    const lower = file.toLowerCase();
    if (
      lower.startsWith("license") ||
      lower.startsWith("licence") ||
      lower.startsWith("copying")
    ) {
      const licensePath = path.join(pkgDir, file);
      try {
        if (fs.statSync(licensePath).isFile()) {
          const content = fs.readFileSync(licensePath, "utf-8");
          outputContent += `========================================================================
Dependency: cargo - ${pkg.name} (${pkg.version})
File: ${file}
========================================================================
${content}

`;
          licenseFound = true;
        }
      } catch (e) {
        console.error(
          `Failed to read cargo license for ${pkg.name}:`,
          e.message,
        );
      }
    }
  }

  if (!licenseFound) {
    // If no file was found but license type is in metadata
    if (pkg.license) {
      outputContent += `========================================================================
Dependency: cargo - ${pkg.name} (${pkg.version})
License Type: ${pkg.license}
(No license file found in the package)
========================================================================

`;
    }
  }
}

fs.writeFileSync(OUTPUT_FILE, outputContent, "utf-8");
console.log(`Successfully generated licenses.tmp at ${OUTPUT_FILE}`);
