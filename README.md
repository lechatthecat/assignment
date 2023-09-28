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
- Logs of all three containers are created in /logs
- I know login feature is not required, but this app allows the staff to login with JWT token. Once they login, they can use the jwt token to use protected APIs. With this login, the API side doesn't confuse which request is made by who. 
- Html/JS/CSS files are handled by Nginx. Nginx reverse-proxies the requests for API to Actix container. Thus only API requests are handled by Actix. (I know it was not necessary though..)

## ToDo

- Maybe I could have used swagger to create documents for APIs
- I could have created more test cases in Rust code/Actix.
- I know there would have been better way for Nextjs compilation... But this isn't the main task for this time.
  
## Requirements
The [original requirements](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md) are:

- The client (the restaurant staff “devices” making the requests) MUST - be able to: add one or more items with a table number, remove an item - for a table, and query the items still remaining for a table.
- The application MUST, upon creation request, store the item, the table - number, and how long the item will take to cook.
- The application MUST, upon deletion request, remove a specified item - for a specified table number.
- The application MUST, upon query request, show all items for a - specified table number.
- The application MUST, upon query request, show a specified item for a - specified table number.
- The application MUST accept at least 10 simultaneous incoming add/- remove/query requests.
- The client MAY limit the number of specific tables in its requests to - a finite set (at least 100).
- The application MAY assign a length of time for the item to prepare as - a random time between 5-15 minutes.
- The application MAY keep the length of time for the item to prepare static (in other words, the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion).

