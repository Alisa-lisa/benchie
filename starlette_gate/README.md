# Mock of a destination recommender
- starlette
- asynchttp

to build and run:
- make sure you have a docker network `foo`: `docker network create foo`

```docker build -t gate .```

```docker run -p 5002:5002 --network=foo gate```

You might run into problems with url of the proxy service. use proxy ip from foo network
`docker network inspect foo`

##### Stress test
- no data in proxy, same request (system has enough free resources, duh)

`wrk -c10 -t1 -d30s "http://localhost:5002/123/London"`

```
Running 30s test @ http://localhost:5002/123/London
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.85ms    5.81ms  81.86ms   80.92%
    Req/Sec   405.60     54.87   505.00     73.33%
  12151 requests in 30.09s, 2.46MB read
Requests/sec:    403.83
Transfer/sec:     83.72KB
```