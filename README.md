## How to use this application

### Prerequisites
- Docker
- Docker-compose

## How to start this application
```shell
$ git clone https://github.com/lechatthecat/assignment
$ cd assignment
$ docker-compose up --build -d
```

After the compilation of the docker-compose environment, the application should be running on: http://localhost/login

You can login with:  
name: test_user1  
password: password  

DB schema initialization, DB records initialization are automatically done by the docker-compose.

## How to test this app
You can see the test code in:
simple_restaurant_api/src/main.rs

Please go to the root directory of where you cloned the assignment repository. Then run:
```shell
$ cd simple_restaurant_api
$ cargo test
```

Then the test code will asynchronously send various requests to the API server.
The requests are:
1. Random user will login
2. Get all tables information (and the staff goes to check orders to a table)
3. Get menu items (and show it to the customer)
4. Pick three menu items and see detail
5. Order the three items
6. Cancel one of them
7. Serve one of them
8. Delete all orders of the table

This set of requests will be sent 15 times at once asynchronously.
As actix web server will spawn 20 workers, it should handle more than 10 requests at once.
So I think this will achieve this requirement:

> The application MUST accept at least 10 simultaneous incoming add/- remove/query requests.

## But will it really work as an application?
I know separate frontend is not necessary at all. But I prepared a very simple frontend with React and Next.js. You can see the login page from here: http://localhost/login

## About the application
- This app creates the enviroment by docker-compose
- It has 3 containers: Rust (Actix) container, Postgresql container, Nginx container
- Docker related files are in: /docker
- Logs of DB and Nginx containers are created in /logs
- Log of Rust (API) container is created in /simple_restaurant_api/log
- I know login feature is not required, but this app allows the staff to login with JWT token. Once they login, they can use the jwt token to use protected APIs. With this login, the API side doesn't confuse which request is made by who. 
- Html/JS/CSS files are handled by Nginx. Nginx reverse-proxies the requests for API to Actix container. Thus only API requests are handled by Actix. (I know it was not necessary though..)

## ToDo

- Maybe I could have used swagger to create documents for APIs
- I could have created more test cases in Rust code/Actix.
- I know there would have been better way for Nextjs compilation... But this isn't the main task for this time.
  