import * as semver from 'semver';
import simpleGit, { SimpleGit } from 'simple-git';
import { Octokit } from '@octokit/core';
import { PaginateInterface } from '@octokit/plugin-paginate-rest';

interface Label {
    id?: number;
    node_id?: string;
    url?: string;
    name?: string;
    description?: string;
    color?: string;
    default?: boolean;
}
interface PullRequest { number: number, title: string, body: string, labels: Label[] }
type PullRequests = Map<string, PullRequest>;
type OctokitPlusExtra = Octokit & { paginate: PaginateInterface };

/**
 * Generate content for a markdown document with all release note entries. Set
 * the `nextVersion` to replace the "Unreleased" entry with a version and
 * today's date.
 */
export async function generate(octokit: OctokitPlusExtra, owner: string, repo: string, nextVersion: string | undefined): Promise<string> {
    const git = simpleGit();

    const logEntries = (await gatherLogEntries(git, octokit, owner, repo, nextVersion)).reverse();

    const linkedIssues = new Set<number>();
    const linkedPullRequests = new Set<number>();

    let output = "# Changelog";

    let lastVersionIsEmpty = false;

    for (const entry of logEntries) {
        output = withDoubleNewline(output);

        if (entry.version) {
            output += formatVersionHeading(entry.version, entry.date);
        } else {
            output += '## Unreleased';
        }

        output = withDoubleNewline(output);

        lastVersionIsEmpty = !entry.pullRequests.length;
        for (const pullRequest of entry.pullRequests) {
            linkedPullRequests.add(pullRequest.number);

            const closedIssues = pullRequest.body
                .toLowerCase()
                .match(/(close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved) #[0-9]+/g)
                ?.map((closedIssue) => {
                    const number = parseInt(closedIssue.match(/#([0-9]+)/)![1]);
                    linkedIssues.add(number);
                    return formatIssueLink(number);
                });

            output = withNewline(output);
            output += formatLogEntry(pullRequest.number, pullRequest.title, closedIssues);
        }
    }

    if (lastVersionIsEmpty) {
        output = withDoubleNewline(output);
        output += "The first published version."
    }

    output = withDoubleNewline(output);

    const sortedPullRequests = [...linkedPullRequests.values()].sort((a, b) => a - b);
    for (const pullRequest of sortedPullRequests) {
        output = withNewline(output);
        output += formatPullRequestReference(owner, repo, pullRequest);
    }

    const sortedIssues = [...linkedIssues.values()].sort((a, b) => a - b);
    for (const issue of sortedIssues) {
        output = withNewline(output);
        output += formatIssueReference(owner, repo, issue);
    }

    output = withNewline(output);
    return output;
}

/**
 * Gather all release note entries for a repository.
 */
async function gatherLogEntries(git: SimpleGit, octokit: OctokitPlusExtra, owner: string, repo: string, nextVersion: string | undefined) {
    const versionTags = (await git.tags()).all.filter((tag) => semver.valid(tag));
    versionTags.sort(semver.compare);

    const mergedPullRequests: PullRequests = new Map();

    const allPullRequests = await octokit.paginate('GET /repos/{owner}/{repo}/pulls', {
        owner,
        repo,
        state: 'closed',
    });

    for (const pullRequest of allPullRequests) {
        const { merged_at, number, title, body, labels } = pullRequest;

        if (!merged_at) {
            continue;
        }

        mergedPullRequests.set(number.toString(), { number, title, body: body ?? "", labels });
    }

    const firstVersion = versionTags.length ? versionTags[0] : undefined;
    const lastVersion = versionTags.length ? versionTags[versionTags.length - 1] : undefined;
    const logEntries = [];

    if (firstVersion) {
        const releaseCommit = await getReleaseCommit(git, firstVersion);

        logEntries.push({
            version: firstVersion,
            date: new Date(releaseCommit.date),
            pullRequests: await gatherPullRequests(git, undefined, firstVersion, mergedPullRequests)
        });
    }

    for (var i = 0; i < versionTags.length - 1; i++) {
        const from = versionTags[i];
        const to = versionTags[i + 1];

        const releaseCommit = await getReleaseCommit(git, to);

        logEntries.push({
            version: to,
            date: new Date(releaseCommit.date),
            pullRequests: await gatherPullRequests(git, from, to, mergedPullRequests)
        });
    }

    if (lastVersion) {
        logEntries.push({
            version: nextVersion,
            date: new Date(),
            pullRequests: await gatherPullRequests(git, lastVersion, 'HEAD', mergedPullRequests)
        });
    }

    return logEntries;
}

/**
 * Gather all merged, non-internal pull requests between two versions.
 */
async function gatherPullRequests(git: SimpleGit, from: string | undefined, to: string, pullRequests: PullRequests) {

    const commits = from
        ? (await git.log({ from, to, '--merges': undefined })).all
        : (await git.log({ symmetric: true, [to]: undefined, '--merges': undefined })).all;

    return commits
        .map((commit) => {
            const matches = commit.message.match(/#([0-9]+)/);

            if (!matches) {
                return undefined;
            }

            const pullRequest = pullRequests.get(matches[1]);

            if (!pullRequest || pullRequest.labels.some((label) => label.name?.toLowerCase() == "internal")) {
                return undefined;
            }

            return pullRequest;
        })
        .filter<PullRequest>((pullRequest): pullRequest is PullRequest => !!pullRequest);
}

/**
 * Get the git log entry for the commit a version number tag points at. Throws an error if it's not found.
 */
async function getReleaseCommit(git: SimpleGit, version: string) {
    const releaseCommit = (await git.log({ [version]: undefined })).latest;

    if (!releaseCommit) {
        throw new Error(`missing release commit for ${version}`);
    }

    return releaseCommit;
}


/**
 * Format the heading for a version section, including version number and release date.
 */
function formatVersionHeading(version: string, date: Date): string {
    const formattedDate = `${date.getUTCFullYear()}-${(padNumber(date.getUTCMonth() + 1, 2))}-${padNumber(date.getUTCDate(), 2)}`;
    return `## Version ${version} - ${formattedDate}`;
}

/**
 * Format a release log entry, with number, title and closed issues.
 */
function formatLogEntry(number: number, title: String, closedIssues: string[] | undefined): string {
    let formattedTitle = title.trim();
    if (!formattedTitle.endsWith('.')) {
        formattedTitle = formattedTitle + '.'
    }

    let result = `* ${formatIssueLink(number)}: ${formattedTitle}`;

    if (closedIssues && closedIssues.length) {
        result += ` Closes ${closedIssues.join(', ')}.`;
    }

    return result;
}

/**
 * Format a link to a Github issue or pull request, referred to by its number.
 * This link should be paired with a reference in the same document.
 */
function formatIssueLink(number: number): string {
    return `[#${number}][${number}]`;
}

/**
 * Format a reference to a Github issue, referred to by its number.
 */
function formatIssueReference(owner: string, repo: string, number: number): string {
    return `[${number}]: https://github.com/${owner}/${repo}/issues/${number}`;
}

/**
 * Format a reference to a Github pull request, referred to by its number.
 */
function formatPullRequestReference(owner: string, repo: string, number: number): string {
    return `[${number}]: https://github.com/${owner}/${repo}/pull/${number}`;
}


/**
 * Returns the input string with at least one `\n` at the end, unless it's empty.
 */
function withNewline(text: string): string {
    if (text.length && !text.endsWith('\n')) {
        return text + '\n';
    }

    return text;
}

/**
 * Returns the input string with at least two `\n` at the end, unless it's empty.
 */
function withDoubleNewline(text: string): string {
    while (text.length && !text.endsWith('\n\n')) {
        text += '\n';
    }

    return text;
}

/**
 * Adds 0 padding to the input number, up to `length`.
 */
function padNumber(number: number | string, length: number): string {
    let result = number.toString();

    while (result.length < length) {
        result = '0' + result;
    }

    return result;
}
