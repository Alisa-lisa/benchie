# Proxy service to query python
1. Starlette build upon asyncpg
2. Postgres 10+

### simple test results
- no data in the table, same request, no cache
```wrk -c10 -t1 -d30s "http://localhost:5003/123/London"```
```
Running 30s test @ http://localhost:5003/123/London
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.38ms   10.53ms 156.80ms   92.62%
    Req/Sec   769.57    242.24     1.15k    70.67%
  23021 requests in 30.06s, 3.10MB read
  Requests/sec:    765.79
  Transfer/sec:    105.45KB
``` 
- 1k different data points, different urls
