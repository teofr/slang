#!/bin/bash

# check-base-report.sh: finds the base branch commit that a PR benchmark has to be compared against, and checks
# that bencher already has a report for it, so it can be passed to 'bencher run --start-point-hash'.
#
# Without it, bencher compares against the base branch's latest report at upload time, which is often not the
# commit the PR was merged with: any newer (or missing) base branch commits would be attributed to the PR. And
# bencher silently falls back to that latest report when the start point hash has none, hence this check.
#
# usage: check-base-report.sh project
#
# It runs in a 'pull_request' job, checked out at GitHub's test merge commit (the PR head merged into the base
# branch as it was when the event fired), and reads the job's 'GITHUB_*' variables, 'BENCHER_PR_HEAD_HASH' (the
# PR head), and 'GH_TOKEN' (to link the run to wait for). On success, it writes 'hash=<base commit>' to
# 'GITHUB_OUTPUT'. Otherwise, it fails the job with a message telling what to wait for or re-run.

set -euo pipefail

project="$1"
testbed="ci"
api="https://api.bencher.dev/v0/projects/${project}"

base_ref="${GITHUB_BASE_REF:?}"
repository="${GITHUB_REPOSITORY:?}"
server_url="${GITHUB_SERVER_URL:?}"
workflow_ref="${GITHUB_WORKFLOW_REF:?}"
output="${GITHUB_OUTPUT:?}"
pr_head_hash="${BENCHER_PR_HEAD_HASH:?}"

function fail() {
  echo "::error::$1"
  exit 1
}

# The test merge commit's first parent is the base branch commit, and its second one the PR head:
if ! git rev-parse --verify --quiet "HEAD^2" > /dev/null || git rev-parse --verify --quiet "HEAD^3" > /dev/null; then
  fail "Expected HEAD to be GitHub's test merge commit of this PR, but it isn't a merge of two commits."
fi
merged_hash="$(git rev-parse "HEAD^2")"
if [[ "${merged_hash}" != "${pr_head_hash}" ]]; then
  fail "Expected the second parent of the test merge commit to be the PR head ${pr_head_hash}, but it is ${merged_hash}."
fi

base_hash="$(git rev-parse "HEAD^1")"
echo "Comparing against '${base_ref}' at ${base_hash}."

# The projects are public, so their reports can be listed without an API key. They are listed newest first, so
# the base commit's report is near the top. 255 is the largest page. An unknown branch gives an empty list.
if ! reports="$(curl --silent --show-error --fail --retry 3 --get \
  --data-urlencode "branch=${base_ref}" \
  --data-urlencode "testbed=${testbed}" \
  --data "per_page=255" \
  --data "sort=date_time" \
  --data "direction=desc" \
  "${api}/reports")"; then
  fail "Couldn't list the '${base_ref}' reports of '${project}' on bencher. Re-run this job later."
fi

branch_uuid="$(jq --raw-output '.[0].branch.uuid // empty' <<< "${reports}")"

if [[ -n "${branch_uuid}" ]]; then
  # Bencher resolves the start point hash only among the versions of the base branch's current head. Each run of
  # a PR benchmark starts a new head for its branch ('--start-point-reset'), so a stacked PR's base branch can
  # have a report for the commit in an older head, which bencher would ignore. The 'main' head never changes.
  # (The base branch can still get a new head between this check and the upload, if its PR is benchmarked again
  # in the meantime: that rare race still falls back to its latest report.)
  if ! head_uuid="$(curl --silent --show-error --fail --retry 3 "${api}/branches/${branch_uuid}" | jq --raw-output '.head.uuid')"; then
    fail "Couldn't read the '${base_ref}' branch of '${project}' on bencher. Re-run this job later."
  fi

  if jq --exit-status --arg hash "${base_hash}" --arg head "${head_uuid}" \
    'any(.[]; .branch.head.version.hash == $hash and .branch.head.uuid == $head)' <<< "${reports}" > /dev/null; then
    echo "Found the '${base_ref}' report for ${base_hash}."
    echo "hash=${base_hash}" >> "${output}"
    exit 0
  fi
fi

# No usable report: find the run of this workflow for the base commit, to link it. 'head_sha' is the pushed commit
# for 'push' runs (main), and the PR head for 'pull_request' runs (the base PR of a stacked PR). Runs are listed
# newest first. Manual runs are skipped, as they don't report for that branch. This is best effort: if the lookup
# fails, the message just has no link.
workflow_file="${workflow_ref%%@*}"
workflow_file="${workflow_file##*/}"

run="$(gh api "repos/${repository}/actions/workflows/${workflow_file}/runs?head_sha=${base_hash}&per_page=100" \
  --jq '[.workflow_runs[] | select(.event == "push" or .event == "pull_request")] | first | select(. != null) | [.status, (.conclusion // "none"), .html_url] | @tsv' \
  || true)"

# (No field is ever empty: tabs are whitespace to 'read', so consecutive ones would be merged.)
IFS=$'\t' read -r run_status run_conclusion run_url <<< "${run}" || true

missing="'${base_ref}' has no '${testbed}' benchmark report on bencher for ${base_hash}, which this PR was merged with to be measured. Comparing against any other report would attribute its differences to this PR."

if [[ -n "${run_url:-}" && "${run_status}" != "completed" ]]; then
  fail "${missing} Its benchmark run is still ${run_status//_/ }: wait for ${run_url} to finish, then re-run this job."
fi

if [[ -n "${run_url:-}" ]]; then
  found="Its benchmark run ${run_url} ended with '${run_conclusion}', without a report usable here (each new benchmark of a PR branch replaces its previous reports)."
else
  found="No '${workflow_file}' run was found for it: see ${server_url}/${repository}/commit/${base_hash}."
fi

fail "${missing} ${found} Re-run that benchmark (for a stacked PR, the base PR has to run it on its current head, e.g. with the 'ci:perf' label), or rebase onto a benchmarked commit, then re-run this job."
