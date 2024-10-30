
# Use a minimal base image, like Debian or Alpine
FROM docker.io/library/rust:1.75.0-bookworm

# Copy the pre-built binary from the devcontainer
COPY ../target/release/my-app-2 /usr/local/bin/my-app

# Expose ports if necessary
# EXPOSE 8080

# Set the entrypoint to the binary
CMD ["/usr/local/bin/my-app"]
