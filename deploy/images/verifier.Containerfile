# gateway-verifier-host finds gateway-verifier beside itself. LexLean takes lean and lake from
# $ELAN_HOME/toolchains, so the toolchain needs no elan and no network when it runs (IM-02).
FROM docker.io/library/ubuntu:24.04@sha256:fe81560990fae86dc4743202b2cb4d3df98d0bcf949173cecef15ce1127ef852
COPY gateway-verifier-host gateway-verifier /usr/local/bin/
COPY toolchain /opt/elan/toolchains/leanprover--lean4---v4.32.1
ENV ELAN_HOME=/opt/elan
ENV HOME=/tmp
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/gateway-verifier-host"]
