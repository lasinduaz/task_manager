#Define the enviroment 
FROM rust:1.89

#file store file
WORKDIR /task_manager

#Coppy files
COPY Cargo.lock Cargo.toml /task_manager/

#coppy files 
COPY  src ./src

#Run the program
Run  cargo build && cargo run  

CMD ["sh", "-c", "cargo build && cargo run"]