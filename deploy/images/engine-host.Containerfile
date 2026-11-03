# The Engine calls need no Lean, so this image has none (IM-01).
FROM docker.io/library/ubuntu:24.04@sha256:fe81560990fae86dc4743202b2cb4d3df98d0bcf949173cecef15ce1127ef852
COPY gateway-engine-host /usr/local/bin/gateway-engine-host
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/gateway-engine-host"]
