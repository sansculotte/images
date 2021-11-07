Images
======

Serve Images, create and store thumbnails


Development
-----------


Deployment
----------

- pull/clone repository
- docker-compose up -d

Optionally proxy through nginx, add a vhost definition like this:

```
server {
    listen *:80;

    server_name           <your.server.name>;

    access_log            /var/log/nginx/access.log combined;
    error_log             /var/log/nginx/error.log;

    location / {
        proxy_pass            http://localhost:8000;
        proxy_read_timeout    90s;
        proxy_connect_timeout 90s;
        proxy_set_header      Host $host;
        proxy_set_header      X-Real-IP $remote_addr;
        proxy_set_header      X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header      Proxy "";
    }
}
```
