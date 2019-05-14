run:
	docker-compose up -d --build postgres
	sleep 3
	docker-compose up --build gate

down:
	docker-compose down
	docker system prune