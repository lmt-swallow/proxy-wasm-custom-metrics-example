# A sample implementation of an Envoy filter with custom metrics

You can run Envoy with custom metrics by the following commands:

```shell
$ docker-compose up
(snip...)
```

Then you can find two custom metrics are added to Envoy:

```shell
$ curl -s localhost:9901/stats | grep stat_filter
stat_filter.custom_metric.num_of_request: 0
stat_filter.custom_metric.num_of_response: 0
```

Once you send requests for `http://localhost:8080`, those metrics will be incremented:

```shell
$ curl http://localhost:8080
hello from mock
$ curl -s localhost:9901/stats | grep stat_filter
stat_filter.custom_metric.num_of_request: 1
stat_filter.custom_metric.num_of_response: 1
```