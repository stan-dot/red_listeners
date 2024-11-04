
# Use a minimal base image, like Debian or Alpine
FROM docker.io/library/rust:1.75.0-bookworm as build

# Copy the pre-built binary from the devcontainer
COPY ../target/release/my-app /usr/local/bin/my-app

# Expose ports if necessary
# EXPOSE 8080

# Set the entrypoint to the binary
CMD ["/usr/local/bin/my-app"]

FROM gcr.io/distroless/cc-debian12@sha256:e6ae66a5a343d7112167f9117c4e630cfffcd80db44e44302759ec13ddd2d22b AS deploy

COPY --from=build /usr/local/bin/my-app /app

ENTRYPOINT ["/bundler"]
