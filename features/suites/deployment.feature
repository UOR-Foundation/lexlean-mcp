Feature: The loopback deployment

  The services of issue #17 as one PrismPM host-effect system model, mcp-loopback, from which PrismPM projects the Compose project. The model is checked here; its projection and its run are not.

  @DP-01 @build
  Scenario: Production.Core and Production.HostEffectSystem of this repository are, byte for byte, the two standard library files of PrismPM ecb594bd: the SHA-256 over their path-sorted manifest equals the checksum of the authority A-PRISMPM-SYSTEM.
    Given src/Production/Core.lex.tex, src/Production/HostEffectSystem.lex.tex and the authority A-PRISMPM-SYSTEM
    When the manifest of their SHA-256 digests is hashed
    Then the digest equals the checksum of the authority, which names PrismPM ecb594bd

  @DP-02 @build
  Scenario: The deployment mcp-loopback is a Production.HostEffectSystem system model whose only api component is the gateway, which declares no network and owns the one listener, on 127.0.0.1 at the owner value listen.port; every other component is on the internal network backend alone, the signing key is the signer's alone, and lexlean verify proves that validateManifest accepts the model for every build that names its five images and only for such a build.
    Given src/Deployment.lex.tex
    When its components, networks, listener and secrets are read, and its theorems are read with their statements and proofs
    Then the gateway alone is an api on no network, every other component is on backend alone, backend is internal, the listener is the owner value on loopback, the signer alone holds a secret, and the validity theorem equates validateManifest with the five images being named

  @DP-03 @build
  Scenario: The application model digest the deployment binds, in its profile and its manifest, is sha256: followed by the SHA-256 of gateway/src/Application.lex.tex, the declaration of the server application the gateway serves.
    Given src/Deployment.lex.tex and gateway/src/Application.lex.tex
    When the declaration is hashed
    Then gatewayApplicationDigest is that hash, and the profile and the manifest both bind gatewayApplicationDigest

  @DP-04 @build
  Scenario: The effects the deployment declares are the host protocol of the gateway: the 15 effect names of gateway/src/Application.lex.tex, in its order, which begin with the ten names of src/Effects.lex.tex in the order of allEffects.
    Given src/Deployment.lex.tex, gateway/src/Application.lex.tex and src/Effects.lex.tex
    When the host effects of the application and the effects the deployment declares are read in order
    Then they are the same 15 names, and the first ten are the names of allEffects

  @DP-05 @build
  Scenario: gateway-signer ready, gateway-engine-host ready and gateway-verifier-host ready, the readiness commands the deployment names, ask the service at the socket their environment names whether it can serve and exit 0 only on its yes: the signer can load its key, the engine host has its store, the verifier host has its verifier program. With no socket variable, no service at the socket, or a service that says no, each exits 1 and says why; any other argument is a usage error, exit 64.
    Given each of the three programs, its service started on a socket of a temporary directory, and the readiness commands of src/Deployment.lex.tex
    When the program is run with ready against a service that can serve, one that cannot, no service, and no variable, and with another argument
    Then it exits 0 only for the service that can serve, 1 with the reason otherwise, 64 for the other argument, and each readiness command of the model names one of these programs with ready

  @DP-06 @build
  Scenario: gateway-ready, the readiness command of the gateway, connects to the address and port of the gateway's served declaration, surface::ADDRESS and surface::PORT of the generated host, and exits 0 only when the answer begins with an HTTP/1.1 status line; with nothing listening, or an answer that is not HTTP, it exits 1 and says why. The gateway image holds gateway-host and gateway-ready.
    Given a loopback listener that answers with a status line, one that answers with something else, a port with nothing on it, and the source of gateway-ready
    When the probe is run against each
    Then only the status line is ready, the program reads its address and port from the generated surface alone, and the gateway image definition copies gateway-host and gateway-ready

  @DP-07 @build
  Scenario: The signer reads its key from the file LEXLEAN_MCP_SIGNING_KEY_FILE names, the variable PrismPM's projection sets for the secret lexlean-mcp-signing-key, and the verifier host removes that variable from the environment of a verifier process; no source of the gateway names LEXLEAN_MCP_SIGNING_KEY without the suffix _FILE.
    Given the signer and the verifier host, src/Deployment.lex.tex and the sources below gateway/bindings and deploy
    When the variable the projection derives from the secret is computed and the sources are read
    Then it is the variable both programs name, and no source names the variable without its suffix
