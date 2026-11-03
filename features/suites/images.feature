Feature: The images of the services

  The services of issue #17 as images of their own, run apart and exercised through the gateway. This is what the deployment will start; the Compose projection that starts it belongs to PrismPM and is not here.

  @IM-01 @build
  Scenario: Each service has an image of its own, defined in deploy/images: gateway-host, gateway-engine-host, gateway-signer, gateway-verifier-host with gateway-verifier, and the telemetry collector; each definition starts from a base named by its digest, has no RUN step and runs as user 65532, and each image holds its own program and no other service's.
    Given the definitions in deploy/images and the images built from them
    When their bases, steps and users are read, and each image is asked what it holds
    Then every base is a digest, no definition runs a step, and each holds its own program alone

  @IM-02 @build
  Scenario: The verifier image alone holds Lean, at the version gateway/lean-toolchain names, unpacked from the release archive whose SHA-256 deploy/images/lean-toolchain.sha256 records, where LexLean finds it through ELAN_HOME, and an archive of another digest or another version stops the build.
    Given the toolchain the gateway is pinned to and the digest recorded for it
    When the images are built
    Then the verifier image holds that toolchain, checked, and no other image holds any

  @IM-03 @build
  Scenario: The services run apart from their images: each in a container of its own with a read-only root and no capabilities; the engine host, the verifier host and the signer have no network but loopback; each shares with the gateway only its own socket and the store; the key is mounted in the signer alone; and through the gateway a module is checked, built and verified, its signed record is found by its ID, a cancel ends a Lean child of its task, and neither a cancel nor a process killed under Lean leaves a process or a zombie.
    Given the images, each started with only what its service needs
    When a client reaches the gateway and verifies a module
    Then the record is signed and found, and each container has only what it was given

  @IM-04 @build
  Scenario: The collector of its image receives what the gateway exports: an alert reaches it from the telemetry file, and no text of a module does.
    Given the gateway writing its telemetry file and the collector reading it
    When a failure fires an alert
    Then the collector has received the alert, and the text of the module is in nothing it received

  @IM-05 @build
  Scenario: A service that stops is a typed failure of the gateway and not a hang: a task in flight when the verifier host stops ends as verifier-lost, verify is refused with verifier-unavailable when the verifier host or the signer is stopped, and an Engine call with engine-unavailable when the engine host is; and a run whose checks did not all pass, or whose count is not the number written in deploy/images/run.sh, fails the gate.
    Given the services running from their images
    When each is stopped
    Then the gateway answers each request with its failure, and the gate holds the run to its count

  @IM-06 @build
  Scenario: Each gate of the images rejects its planted defect: a base named by a tag, a RUN step, a root user, a recorded digest that is not the archive's, a toolchain of another version, Lean in the gateway image, the key mounted in the verifier, a network given to the services, no init to reap what a killed process leaves, a run that stops early, the collector reading another file, and a stopped service left running; IMAGES.md records the gate and the diagnostic of each.
    Given each defect planted alone
    When the gate that holds it runs
    Then it fails, naming the defect, and passes again when the defect is gone
