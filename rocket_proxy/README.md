# Proxy service to query postgres

- Rocket
- Diesel


#### simple test results
- no data in the table, same request, no cache `wrk -c10 -t1 -d30s "http://localhost:8089/123/London"`
```
Running 30s test @ http://localhost:8089/123/London
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    25.21ms  103.54ms   1.01s    97.08%
    Req/Sec     1.11k   235.94     1.41k    83.78%
  16377 requests in 30.07s, 2.16MB read
  Socket errors: connect 0, read 16377, write 0, timeout 0
Requests/sec:    544.71
Transfer/sec:     73.41KB
```
