# build_notifications

Because of the fact that recently I am running many builds I've created this simple app for getting system notifications if there are any new builds on jenkins. Run it once and it will start checking for updates every 3 minutes.

If you run it with `-c` option it will open config.

## Example config

```toml
username = 'jenkins_username'
password = 'token_api'
jenkins_jobs = ['https://jenkins.******.com/job/***job_name***/rssAll','https://jenkins.******.com/job/***other_job_name***/rssAll']
```
