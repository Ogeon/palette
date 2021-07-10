import * as core from '@actions/core';
import { increment, isDependencies } from './increment';

const version = core.getInput('version', { required: true });
const crate = core.getInput('crate', { required: false });
const dependenciesJSON = core.getInput('dependencies');

(async () => {
    let dependencies: unknown;

    try {
        dependencies = dependenciesJSON.length ? JSON.parse(dependenciesJSON) : {};
    } catch (error) {
        core.setFailed(`could not parse dependencies as JSON: ${error}`);
        return;
    }

    if (!isDependencies(dependencies)) {
        core.setFailed(`expected dependencies as JSON data: {"example_crate": "1.2.3"}`);
        return;
    }
    try {
        await increment(crate.length ? crate : undefined, version, dependencies);
    } catch (error) {
        core.setFailed(`could not increment versions: ${error}`);
    }
})();
