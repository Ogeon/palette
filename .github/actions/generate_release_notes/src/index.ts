import * as core from '@actions/core';
import * as github from '@actions/github';
import * as fs from 'fs/promises';
import { generate } from './generate';

const token = core.getInput('token', { required: true });
const version = core.getInput('version', { required: true });
const outFile = core.getInput('file', { required: true });
const { owner, repo } = github.context.repo;

const octokit = github.getOctokit(token);

(async () => {
    try {
        const output = await generate(octokit, owner, repo, version);
        await fs.writeFile(outFile, output);
    } catch (error) {
        core.setFailed(`could not generate release notes: ${error}`);
    }
})()


