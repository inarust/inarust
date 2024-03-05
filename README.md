# inarust

Inarust Boilerplate

test dengan curl

```sh
curl http://localhost:3000/
curl http://localhost:3000/users
curl -X POST http://localhost:3000/create-user
curl "http://localhost:3000/item/42?number=2"
curl -X POST http://localhost:3000/add-item \
    -H "Content-Type: application/json" \
    -d '{"title": "Some random item"}'
curl -X DELETE http://localhost:3000/delete-user/1
curl -X DELETE http://localhost:3000/delete-user/2
```
