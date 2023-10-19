## To build:
```
make build
```

## To run:
```
make run
```

```bash
curl -X POST -d '{"tx_id": "1", "policy_id": "1"}' -H "Content-type: application/json" http://localhost:9090/start-server

curl -X POST -d '{"tx_id": "1", "policy_id": "1", "to": "0.0.0.0:10501"}' -H "Content-type: application/json" http://localhost:9090/start-client

curl -X GET -d '{"tx_id": "1"}' -H "Content-type: application/json" http://localhost:9090/proof
```