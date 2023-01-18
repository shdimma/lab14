Web-server with Axum and nginx proxy

1. Install nginx
    sudo pacman -S nginx

2. Install Docker
    sudo pacman -S docker 
	sudo pacman -S docker-compose

3. Launch
    sudo docker-compose build 
    sudo docker-compose up -d

4. Test
    curl http://localhost/v1/todos/63443a02-2137-48e8-8db5-79fa54e8bfdf
