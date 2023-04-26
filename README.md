# Learning Log - Shakeel Gani
https://github.com/Shak1011/TKS-APP
## Entry 1 - 03/03/2023
For this assignment I have decided I want to familiarise myself with app development something in which I have very little experience in. Therefore I have spearheaded a project to create a ranking app for all badminton players in durham with the goal to give each player an everchanging rating depending on your wins and loses. 

To complete this task I have decided that I need to branch out and learn another language especially one that has me thinking more about memory management as coding in python, the language I have used the most, allowed me not to even have to think about it. Consequently, I have decided to learn rust for the back end due to it being fast, memory-efficient, and the fact it easily interacts with other languages. For the front end I will most likely use javascript.

To start learning Rust I have decided to consult many youtube videos to teach me the very basics. 

### Objectives For Next Week
* Finalise design of badminton ranking app 
* Divide work equally among the group giving us all set goals
* Continue learning the basics of rust
* Learn an efficient way to connect the front-end JavaScript to the back-end Rust

## Entry 2 - 03/03/2023
This week we finalized a group and plan. I will be developing the Log In and Register screen in HTML and Javascript. Furthermore I will be setting up a server using Node Js. I will use Rust to append users into the database due to its high efficiency in data managment and I will also use Rust to hash the passwords. This means I will have to learn how to call Rust code from Javascript and pass information between the two. This is something I have never done before only ever coding in one langauge so it will be a new experience for me. 

In terms of learning Rust this week I have learnt how to do the basics such a setting up a rust file using cargo build. Furthermore, I have also been learning how variables and data types work in Rust (which is significantly harder than python due to having to specify the data type of the variable). I also managed to figure out how inputs work as well as basic arthmetics and type casting. This was done by me following this Youtube tutorial, https://www.youtube.com/watch?v=T_KrYLW4jw8&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ , and the result of it can be seen in my Rust Tutorials file. 

### Objectives For Next Week
* Set Up HTML page 
* Continue learning the basics of Rust
* Learn Hashing with Rust
* Set up a NodeJs server

## Entry 3 - 10/03/2023
This week I decided to take a break from learning Rust and therefore focused on the client side aspect of it. For that reason I spent my time creating the HTML page for the Log In system. I also created the JavaScript code to send information from the client to to the server once the server was set up and running. For the time being I just passed the information from the HTML forms as Json information as I have not yet managed to figure out how to run rust code within javascript. This was significantly easier for me as I used my experience from the previous coursework to do it.

### Objectives For Next Week
* Set up a NodeJs server
* Finish learning the basics of Rust
* Learn Hashing with Rust
* Learn how to intergrate rust code within Javascript

## Entry 4 - 17/03/2023
This week I decided to go back to learning Rust and finally finished the basics. This involved everything from me learning about conditions to how to access and use functions in Rust. I also learned how to import on Rust and this allowed me to access Rust code I had written in other files. Furthermore, I learnt more about the Cargo.toml file which allowed me to compile my rust code into dynamic libraries which not only makes the code much more efficient when running but also is the first steps towards being able to use Rust in Javascript.

I also learnt how to use the SQLX import in Rust which allows me to create SQL databases in Rust via the use of SQL queries. This not only means that after brushing on up on my SQL I managed to create a database with a user table and input information into it. I also finished setting up the NodeJS server and currently have data from my HTML forms being appended into a JSON file on the server side.

### Objectives For Next Week
* Learn Hashing with Rust
* Learn how to intergrate rust code within Javascript

## Entry 5- 07/04/2023
This week I finished my Hashing algorithm by using SHA-2. However rather than a file I just Hash the password the user entered then converted the output from hex to characters and the characters are what I store in the database. This can be seen in the file Rust Tutorial under the title Hashing.

### Objectives For Next Week
* Learn how to intergrate rust code within Javascript

## Entry 6- 26/04/2023
This week I finally learnt how to intergrate Rust into Javascript following this youtube video: https://www.youtube.com/watch?v=kkc2Z_PI8E8&t=806s
This can be seen in the Rust Tutorial file. I then used the information I learnt from this to finish of my nodejs server finally completing most of the goals I set. In reality I'd like to neaten up how my server works and also produce more messages to the users if people try to create an account if an account with that email already.
Howevever I can now say I have managed to complete all my objectives I set out. I have created a Log In Page in HTML and Javascript using nodejs that allows for a user to create an account which is appended into an SQL database and allows a user to Log in and recieve the information about stored about themselves on the client side allowing my partners to use this for the rest of the website
