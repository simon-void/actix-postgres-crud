FROM ubuntu

# any following command is executed relative to this path
WORKDIR /app

# copy from_path_outside_container_relative_to_buildCmd to_path_inside_container
# to e.g. include configuration files
# first ./ is current folder, ./ is relative to work directory inside the container
COPY ./target/release/actix-postgres-crud ./

# Start command
CMD ["./actix-postgres-crud"]