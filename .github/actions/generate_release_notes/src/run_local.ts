import { Octokit } from '@octokit/core'
import { generate } from './generate';
import { paginateRest } from '@octokit/plugin-paginate-rest';

const octokit = new (Octokit.plugin(paginateRest));
const owner = process.argv[2];
const repo = process.argv[3];
const nextVersion = process.argv[4]

if (!owner || !repo) {
    console.error(`usage: ${process.argv[0]} ${process.argv[1]} OwnerName RepoName [1.2.3]`);
    process.exit(1);
}

(async () => {
    try {
        const output = await generate(octokit, owner, repo, nextVersion);
        console.log(output);
    } catch (error) {
        console.error(`could not generate release notes: ${error}`);
        process.exit(1);
    }
})()
