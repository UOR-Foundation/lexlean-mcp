import { test } from 'node:test';
import assert from 'node:assert/strict';
import { allowedTypes, checkPullRequest, subjectProblem } from './conventional-commits.mjs';

test('the subjects this repository already writes conform', () => {
  for (const subject of [
    'docs(deploy): a release root waits on the server application of PrismPM#342 (Part of #17)',
    'style(gateway): rustfmt the test the key variable\'s rename lengthened',
    'fix(drafts): a file of a draft replaces the profile\'s file at its path (Fixes #84) (#85)',
    'feat(model): prismpm check passes on the model and runs in just vv (Part of #5, #14, #15, #18) (#79)',
    'build(sdk): development binding to the PrismPM main e84124f0 candidate (Part of #4) (#77)',
    'ci(gateway): free runner disk before pulling the SDK image (#72)',
    'feat(gateway)!: a breaking change to the tool layer',
    'chore(main): release 0.1.0',
    'chore(release): update Cargo.lock',
    'revert: feat(gateway): a breaking change to the tool layer',
  ]) assert.equal(subjectProblem(subject), null, subject);
});

test('a free-form subject, an unknown type and a malformed scope are refused', () => {
  assert.match(subjectProblem('Add the gateway'), /not "type\(scope\): subject"/);
  assert.match(subjectProblem('wip: release-please config'), /"wip" is not a type/);
  assert.match(subjectProblem('feat(Gateway): capital scope'), /scope "Gateway"/);
  assert.match(subjectProblem('feat(): empty scope'), /scope ""/);
  assert.match(subjectProblem('feat:  two spaces'), /must start with text/);
  assert.match(subjectProblem('feat: '), /must start with text/);
});

test('the accepted types are the changelog types, so a type cannot reach main and vanish from the log', () => {
  assert.deepEqual(allowedTypes({ 'changelog-sections': [{ type: 'feat' }, { type: 'fix' }, { type: 'fix' }] }), ['feat', 'fix']);
  for (const type of ['feat', 'fix', 'docs', 'ci', 'style', 'chore', 'revert']) assert.ok(allowedTypes().includes(type), type);
});

test('the title always has to conform', () => {
  const { errors } = checkPullRequest({ title: 'Add a thing', commits: ['feat: add a thing', 'fix: and fix it'] });
  assert.equal(errors.length, 1);
  assert.match(errors[0], /pull request title/);
});

test('a lone commit has to conform, because a squash merge keeps its subject over the title', () => {
  const { errors } = checkPullRequest({ title: 'feat(gateway): add a thing', commits: ['add a thing'] });
  assert.equal(errors.length, 1);
  assert.match(errors[0], /only commit/);
  assert.deepEqual(checkPullRequest({ title: 'feat(gateway): add a thing', commits: ['feat(gateway): add a thing\n\nBody.'] }).errors, []);
});

test('merge commits do not count, and other subjects in a longer pull request only warn', () => {
  const merged = checkPullRequest({ title: 'feat: a', commits: ['feat: a', "Merge branch 'main' into topic"] });
  assert.deepEqual(merged, { errors: [], warnings: [] });
  const longer = checkPullRequest({ title: 'feat: a', commits: ['feat: a', 'wip: more'] });
  assert.deepEqual(longer.errors, []);
  assert.equal(longer.warnings.length, 1);
});
