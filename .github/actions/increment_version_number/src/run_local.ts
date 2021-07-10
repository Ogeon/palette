import * as process from 'process';
import { increment, isDependencies } from './increment';

const version = process.argv[2].trim();
const crate = process.argv[3].trim();
const dependenciesJSON = process.argv[4];

(async () => {
    let dependencies: unknown;

    try {
        dependencies = dependenciesJSON.length ? JSON.parse(dependenciesJSON) : {};
    } catch (error) {
        console.error(`could not parse dependencies as JSON: ${error}`);
        return;
    }

    if (!isDependencies(dependencies)) {
        console.error(`expected dependencies as JSON data: {"example_crate": "1.2.3"}`);
        return;
    }
    try {
        await increment(crate.length ? crate : undefined, version, dependencies);
    } catch (error) {
        console.error(`could not increment versions: ${error}`);
    }
})();
