run:
	docker-compose up -d --build postgres
	sleep 3
	docker-compose up --build proxy

down:
	docker-compose down
	docker system prune