### Installing Rust
```
./rush-ubuntu.sh
```

### Production
```
git clone your_project
```
```
cd your_project
```
- release project
```
cargo build --release
```
```
sudo vi /etc/systemd/system/rustapi.service
```
```
[Unit]
Description=ActixWebApi
After=network.target

[Service]
ExecStart=/home/ubuntu/rust-dummy-api-actix-web/target/release/rust-dummy-api-actix-web
User=ubuntu
Group=ubuntu
Restart=always
Environment=RUST_LOG=info
WorkingDirectory=/home/ubuntu/rust-dummy-api-actix-web/target/release

[Install]
WantedBy=multi-user.target
```
- initialize services
```
sudo systemctl daemon-reload
```
```
sudo systemctl enable rustapi.service
```
```
sudo systemctl start rustapi.service
```
- chect status
```
sudo systemctl status rustapi.service
```

### ngxinx proxy pass (ubunt)
- unlink default setting
```
sudo unlink /etc/nginx/sites-enabled/default
```
- create site conf
```
sudo vi /etc/nginx/sites-available/rustapi.conf
```
```
server {
	listen 80;
	listen [::]:80;
	
	#ssl enable or disable by admin
	#listen 443 ssl;
	#server_name mmserver-test.com;
	#ssl_certificate /etc/nginx/certificates/your_rootca.crt;
	#ssl_certificate_key /etc/nginx/certificates/your_rootca.key;
	
	#server root setting by admin with next-js-typescript folder path(2)
	#root /home/ec2-user/site_name_new_v1;
	

	#Normal proxy pass location
	location / {
	# First attempt to serve request as file, then
	# as directory, then fall back to displaying a 404.
	#First attempt to serve request as file, then as directory, then fall back to displaying a 404.

	proxy_pass             http://localhost:3000;        
	proxy_read_timeout     60;
	proxy_connect_timeout  60;
	proxy_redirect         off;
	proxy_buffers 8 16k;
	proxy_buffer_size 32k;

        # Allow the use of websockets
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;

	#origin 404 disable by proxy pass by admin
    	#try_files $uri $uri/ =404;
    }
}
```
- link the active site config
```
sudo ln -s /etc/nginx/sites-available/rustapi.conf  /etc/nginx/sites-enabled/
```
- restart nginx
```
sudo systemctl restart nginx
```
