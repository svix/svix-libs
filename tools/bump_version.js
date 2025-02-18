const { readFileSync, writeFileSync } = require('fs');
const { join } = require('path');
const semver = require('semver')

const versionFilePath = ".version";

const filesPaths = [
    versionFilePath,
    // CLI
    "svix-cli/Cargo.toml",
    "svix-cli/README.md",
    // Rust Server
    "server/svix-server/Cargo.toml",
    // Bridge Server
    "bridge/svix-bridge/Cargo.toml",
    // Rust Client
    "rust/Cargo.toml",
    // CSharp
    "csharp/Svix/Svix.csproj",
    "csharp/Svix/Version.cs",
    // Go
    "go/version.go",
    // Java
    "java/gradle.properties",
    "java/README.md",
    "java/lib/src/main/java/com/svix/Svix.java",
    // Javascript
    "javascript/package.json",
    "javascript/src/request.ts",
    // Kotlin
    "kotlin/gradle.properties",
    "kotlin/README.md",
    "kotlin/lib/src/main/kotlin/Version.kt",
    // Python
    "python/svix/__init__.py",
    // Ruby
    "ruby/Gemfile.lock",
    "ruby/lib/svix/version.rb",
    // OpenAPI spec
    "server/openapi.json",
    // Cloud OpenAPI spec - not necessary but any other time of updating seems weirder
    "lib-openapi.json",
];

const rootDir = join(__dirname, "..");

if (process.argv.length !== 3 || !semver.valid(process.argv[2])) {
    console.error("must supply a valid semantic version number");
    return;
}
const newVersion = process.argv[2];
const currentVersion = readFileSync(join(rootDir, versionFilePath), 'utf8').trim();

if (semver.lte(newVersion, currentVersion)) {
    console.error("supplied version must be greater than current version");
    return;
}

const replaceRegExp = new RegExp(currentVersion, 'g');

// Update Version Files
filesPaths.forEach((relativePath) => {
    const filePath = join(rootDir, relativePath);
    const content = readFileSync(filePath, 'utf8');
    const updated_content = content.replace(replaceRegExp, newVersion);
    writeFileSync(filePath, updated_content);
})

// Add Changelog Section
const changelogPath = join(rootDir, "ChangeLog.md");
const changelogUpdate = readFileSync(changelogPath, 'utf8').replace("# Changelog\n\n## Next", `# Changelog\n\n## Next\n* \n\n## Version ${newVersion}`);
writeFileSync(changelogPath, changelogUpdate);

console.log("Version bumped from %s to %s, don't forget to update the changelog!", currentVersion, newVersion);
