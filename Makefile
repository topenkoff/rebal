clean:
	docker ps -a | grep rebal |  awk '{print $$1}' | xargs docker rm -f \
	    && docker network ls | grep rebal | awk '{print $$1}' | xargs docker network rm

force:
	docker ps -a | grep rebal |  awk '{print $$1}' | xargs docker rm -f \
	    && docker network ls | grep rebal | awk '{print $$1}' | xargs docker network rm  \
	    && docker-compose -p rebal -f dev/docker-compose.yml up --build

run:
	docker-compose -p rebal -f dev/docker-compose.yml up --build

test:
	docker exec -it $$(docker ps | grep test_serv | awk '{print $$1}') python test.py
