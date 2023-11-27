## Adinmo test

### To run with docker
```bash
docker compose up --build -d
```

### To get to app
Its bound to 0.0.0.0:8080 for ease; So you should find it at:
```
127.0.0.1:8080
```

### To query data
First find the db container with
```bash
docker ps
```
Then go to mysql in the container and login with the adinmo user.
```bash
docker exec -it {container_name} mysql -u adinmo -pPassword123!
or
docker exec -it adinmo-test-db-1 mysql -u adinmo -pPassword123!
```

The 2 tables you can query are `post_batch` and `random_number`

```sql
USE adinmo_test_db
SELECT * FROM post_batch;
SELECT * FROM random_number;
```
