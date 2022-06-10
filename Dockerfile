# Use the latest stable release
FROM rust:1.61

# Switch working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# already exist. 
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y 
# copy all files from our working environment to our Docker image
COPY . .
# Build in release to make it fast
RUN cargo build --release
# when `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]