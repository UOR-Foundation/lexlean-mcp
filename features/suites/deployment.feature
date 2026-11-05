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
