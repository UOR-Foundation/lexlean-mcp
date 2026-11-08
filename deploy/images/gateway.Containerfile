# The gateway and its readiness command: no Lean, no key, and no other service's program (IM-01,
# DP-06).
FROM docker.io/library/ubuntu:24.04@sha256:fe81560990fae86dc4743202b2cb4d3df98d0bcf949173cecef15ce1127ef852
COPY gateway-host gateway-ready /usr/local/bin/
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/gateway-host", "serve"]
