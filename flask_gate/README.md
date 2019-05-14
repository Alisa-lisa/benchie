# Mock of a destination recommender

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

```