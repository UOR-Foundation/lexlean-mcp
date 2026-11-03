# The collector is the upstream program at a digest; only its configuration is ours (IM-01).
FROM docker.io/otel/opentelemetry-collector-contrib:0.140.0@sha256:ffc818ee108d0b934fd14207fd87ff247c9b64f344ed349d0b66c166c18a2312
COPY collector.yaml /etc/otelcol-contrib/config.yaml
USER 65532:65532
