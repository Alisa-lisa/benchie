# Proxy service to query python
1. Flask, uWSGI, psycopg2
2. Postgres 10+

### simple test results
- no data in the table, same request, no cache
```wrk -c10 -t1 -d30s "http://localhost:5003/123/London"```
```
Running 30s test @ http://localhost:5003/123/London
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    20.82ms    6.08ms 145.21ms   84.75%
    Req/Sec   472.54     60.71   570.00     75.67%
  14130 requests in 30.03s, 0.98MB read
  Socket errors: connect 0, read 14129, write 0, timeout 0
Requests/sec:    470.55
Transfer/sec:     33.55KB
``` 
- 1k different data points, different urls
