# The key is mounted at run time and is never a layer of this image (IM-01, IM-03).
FROM docker.io/library/ubuntu:24.04@sha256:fe81560990fae86dc4743202b2cb4d3df98d0bcf949173cecef15ce1127ef852
COPY gateway-signer /usr/local/bin/gateway-signer
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/gateway-signer"]
