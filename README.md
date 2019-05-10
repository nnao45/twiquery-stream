[![docker](https://img.shields.io/badge/docker-0.1.0-blue.svg)](https://hub.docker.com/r/nnao45/twiquery-batch/tags)

# twiquery-batch

```shell
$ cat <<EOF > ./.env                                                                                                                                        CONSUMER_KEY: <twitter consumer key>
CONSUMER_SECRET=<twitter consumer secret>
ACCESS_TOKEN=<twitter access token>
ACCESS_TOKEN_SECRET=<twitter access token secret>
TRACK=<twitter search keyword>
SLACK_URL=<post slack webhook url>
IS_DEBUG=<true or false>
EOF
$ make run
```
