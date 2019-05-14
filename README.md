# twiquery-stream

[![docker](https://img.shields.io/badge/docker-0.1.17-blue.svg)](https://hub.docker.com/r/nnao45/twiquery-stream/tags)

[![asciicast](https://asciinema.org/a/Q6bbb19zncsbbEEbHhUM1ngHv.svg)](https://asciinema.org/a/Q6bbb19zncsbbEEbHhUM1ngHv)

```shell
$ cat <<EOF > ./.env                                                                                                                                        CONSUMER_KEY: <
CONSUMER_KEY=<twitter consumer key>
CONSUMER_SECRET=<twitter consumer secret>
ACCESS_TOKEN=<twitter access token>
ACCESS_TOKEN_SECRET=<twitter access token secret>
TRACK=<twitter search keyword>
SLACK_URL=<post slack webhook url>
IS_DEBUG=<true or false>
POST_SLACK_ENABLED=<true or false>
FILTER_LANG=<filtering lang, if noneed, this should be value "none">
EOF
$ make run
```
