import * as core from '@actions/core';
import * as exec from '@actions/exec';
import * as io from '@actions/io';
import * as path from 'path';
import { promises as fs } from 'fs';
import { SemVer } from 'semver';
import { TomlFile } from './edit_toml';

interface Manifest {
    name: string,
    version: string,
    targets: Target[]
}

interface Target {
    name: string,
    doc: boolean,
    src_path: string,
}

interface Dependencies {
    [key: string]: string
}

/**
 * Checks if an unknown value is a dependency map.
 */
export function isDependencies(input: unknown): input is Dependencies {
    if (typeof input !== 'object' || input === null) {
        return false;
    }

    for (const [key, value] of Object.entries(input)) {
        if (typeof key !== 'string' || typeof value !== 'string') {
            return false;
        }
    }

    return true;
}

/**
 * The main business function. Takes care of all the version finding-and-replacing.
 */
export async function increment(cratePath: string | undefined, newVersion: string, dependencies: Dependencies) {
    let manifest: Manifest;
    let tomlFile;

    const manifestPath = cratePath ? path.join(cratePath, 'Cargo.toml') : 'Cargo.toml';
    const readmePath = cratePath ? path.join(cratePath, 'README.md') : 'README.md';

    core.startGroup('Read Cargo manifest');
    try {
        core.info('Find `cargo`');
        const cargoPath = await io.which('cargo');

        core.info('Read manifest as JSON');
        const manifestOutput = await exec.getExecOutput(cargoPath, ['read-manifest'], { cwd: cratePath, silent: true });
        if (manifestOutput.exitCode != 0) {
            throw Error(`error when reading manifest:\n${manifestOutput.stderr}`);
        }

        manifest = JSON.parse(manifestOutput.stdout);


        core.info('Read manifest as editable TOML');
        tomlFile = await TomlFile.load(manifestPath);
    } finally {
        core.endGroup();
    }

    const oldSemver = new SemVer(manifest.version);
    const newSemver = new SemVer(newVersion);
    let oldFormatted = formatVersion(oldSemver);
    let newFormatted = formatVersion(newSemver);


    core.info(`Current version is ${manifest.version}, formatted as ${oldFormatted}`);
    core.info(`New version is ${newVersion}, formatted as ${newFormatted}`);


    core.startGroup('Increment versions in Cargo.toml');
    try {
        core.info(`Set package version to ${newVersion}`);
        tomlFile.setPrimitive(['package', 'version'], newVersion);

        if (tomlFile.hasPrimitive(['package', 'documentation'])) {
            const oldUrl = tomlFile.getPrimitive(['package', 'documentation']);
            if (typeof oldUrl !== 'string') {
                throw new Error(`documentation URL is not a string`);
            }

            const newUrl = oldUrl.replace(manifest.version, newVersion);
            core.info(`Set documentation URL to ${newUrl}`);
            tomlFile.setPrimitive(['package', 'documentation'], newUrl);
        } else {
            core.info('No documentation URL found (skipping)');
        }

        for (let [name, version] of Object.entries(dependencies)) {
            name = name.trim();
            version = version.trim();

            core.info(`Set dependencies.${name} version to ${version}`);

            if (tomlFile.hasPrimitive(['dependencies', name])) {
                tomlFile.setPrimitive(['dependencies', name], version);
            } else if (tomlFile.hasPrimitive(['dependencies', name, 'version'])) {
                tomlFile.setPrimitive(['dependencies', name, 'version'], version);
            } else {
                throw new Error(`dependency ${name} was not found`);
            }
        }

        core.info('Write changes');
        await fs.writeFile(manifestPath, tomlFile.render());
    } finally {
        core.endGroup();
    }


    core.startGroup('Increment versions in documentation link attributes');
    try {
        for (const target of manifest.targets) {
            if (target.doc) {
                let content = (await fs.readFile(target.src_path)).toString('utf-8');
                const matches = content.match(/#\s*!\s*\[\s*doc\s*\(\s*html_root_url\s*=\s*".*"\s*\)\s*\]/g);
                if (matches) {
                    for (const oldLink of matches) {
                        const newLink = oldLink.replace(manifest.version, newVersion);
                        core.info(`Setting documentation link attribute in target ${target.name} to ${newLink}`);
                        content = content.replace(oldLink, newLink);
                    }

                    await fs.writeFile(target.src_path, content);
                } else {
                    core.info(`Found no documentation link in target ${target.name} (skipping)`);
                }
            }
        }
    } finally {
        core.endGroup();
    }


    core.startGroup('Increment versions in README.md');
    try {
        let content = (await fs.readFile(readmePath)).toString('utf-8');
        let changed = false;

        const shortMatches = content.match(new RegExp(`${manifest.name}\\s*=\\s*"${oldFormatted}"`, 'g'));
        if (shortMatches) {
            for (const oldUsage of shortMatches) {
                const newUsage = oldUsage.replace(oldFormatted, newFormatted);
                core.info(`Setting short usages to ${newUsage}`);
                content = content.replace(oldUsage, newUsage);
            }

            changed = true;
        } else {
            core.info(`Found no mentions of ${manifest.name} = "${oldFormatted}" in README.md (skipping)`);
        }

        const longMatches = content.match(new RegExp(`version\\s*=\\s*"${oldFormatted}"`, 'g'));
        if (longMatches) {
            for (const oldUsage of longMatches) {
                const newUsage = oldUsage.replace(oldFormatted, newFormatted);
                core.info(`Setting long usages to ${newUsage}`);
                content = content.replace(oldUsage, newUsage);
            }

            changed = true;
        } else {
            core.info(`Found no mentions of version = "${oldFormatted}" in README.md (skipping)`);
        }

        if (changed) {
            await fs.writeFile(readmePath, content);
        }
    } finally {
        core.endGroup();
    }
}

/**
 * Formats a version number for use as a third party dependency version.
 */
function formatVersion(version: SemVer): string {
    if (version.major === 0) {
        return `0.${version.minor}`;
    }

    return `^${version}`;
}
