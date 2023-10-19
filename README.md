## To run:
```
make run -j2
```

```bash
curl -X POST -d '{"tx_id": "1", "policy_id": "1"}' -H "Content-type: application/json" http://localhost:9090/generate/start-server

curl -X POST -d '{"tx_id": "1", "policy_id": "1", "to": "0.0.0.0:10501"}' -H "Content-type: application/json" http://localhost:9090/generate/start-client
```