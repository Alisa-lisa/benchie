# Proxy service to query postgres (rust)
1. actix-web
2. diesel
2. Postgres 10+

### simple test results
- no data in the table, same request, no cache
```wrk -c10 -t1 -d30s "http://localhost:8089/123/London"```
```
Running 30s test @ http://localhost:8089/123/London
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.99ms    1.40ms  44.40ms   94.43%
    Req/Sec     5.30k   438.18     6.15k    70.00%
  158259 requests in 30.01s, 19.92MB read
Requests/sec:   5274.15
Transfer/sec:    679.87KB
``` 

- 1k different data points, different urls
