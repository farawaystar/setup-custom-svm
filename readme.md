# Cargo.toml Monitor

A GitHub Actions workflow system to monitor and process changes to Cargo.toml files in external repositories. This example will mature into monitoring Solana agave validator client - as a start.

## Workflow Architecture

### 1. Monitoring Workflow (monitor-agave.yml)

*Purpose:* Detect Cargo.toml changes in external repositories
*Trigger Mechanism:* External cron-job.org service initiates this workflow every 5 minutes
- Fetches last 15 commits from target repository
- Compares against last processed commit
- Identifies Cargo.toml modifications using GitHub API
- If changes: triggers artifact generation workflow (process-cargo-changes.yml)
- If no changes: exit

### 2. Processing Workflow (process-cargo-changes.yml)
*Purpose:* Generate consolidated dependency artifacts
*Trigger Mechanism:* Triggered by monitor-agave.yml upon identification of changes to Cargo.toml of the repo being monitored.
- Downloads repository snapshot at specific commit into a timestamped archive.
- Extracts all Cargo.toml files, preserves original directory structure.
- retains artefacts for 3 days


## Configuration Guide

### 1. Personal Access Token (PAT) Setup
**Create Token:**
1. Go to [GitHub Developer Settings](https://github.com/settings/tokens)
2. Create "Classic" token with these scopes:
   - **repo** (Full control)
   - **workflow** (Full control)
   - **admin:org** (Read:org access)

**Token Settings:**
- Expiration: 90 days
- Description: "Agave Monitor Workflow"

### 2. Repository Configuration
**Enable Actions:**
1. Go to `Settings > Actions > General`
2. Set permissions:
   - ✅ Allow all actions & reusable workflows
   - ✅ Read and write permissions
   - ✅ Allow GitHub Actions to create/approve PRs

### 3. Cron-job.org Setup
**Job Configuration:**
Depending on the frequency of monitoring, you can either schedule a cron job using github action's "on schedule" feauture (suitable for 2-3 monitorings per hour) or setup a cron job at cron-job.org (free tier, suitable for more frequent monitoring, upto once per minute)

Following configs apply for cron-job.org setup
1. go to cron-job.org and set up a free tier account
2. Create cron-job. Give following in General and Advance tabs

```
URL: https://api.github.com/repos/farawaystar/agave-monitor/actions/workflows/monitor-agave.yml/dispatches
Method: POST
Headers:
  Authorization: token YOUR_PAT
  Accept: application/vnd.github.v3+json
Body:
{
  "ref": "master"
}
Schedule: */5 * * * *
```
3. Use test-run to test your changes before enabling the job


### 4. Outputs
Find all workflow runs and their statuses in your repo -> Actions


### 5. Verification Commands
**Check Repository Access:**
```
curl -H "Authorization: token YOUR_PAT" \
  https://api.github.com/repos/farawaystar/agave-monitor
```

**Verify Workflow Exists:**
```
curl -L -X GET \
-H "Authorization: token YOUR_PAT" \
-H "Accept: application/vnd.github.v3+json" \
"https://api.github.com/repos/farawaystar/agave-monitor/actions/workflows"
```

## Troubleshooting
**Common Issues:**
- `404 Not Found`: Verify PAT permissions and repository existence
- `Resource not accessible`: Ensure workflow file exists in default branch
- `Authentication failed`: Regenerate PAT with correct scopes

**Debugging Tools:**
```
# Check workflow runs
gh run list -w monitor-agave.yml

# View artifact contents
gh run download  -n agave-cargo-files
```

## License
This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.
```