# TLSN Prover example

This simple example reproduces a problem when a Prover hangs on initialization when running multiple instances.

## Steps to reproduce

1. Start TLSN Notary Server from [tlsn repository](https://github.com/tlsnotary/tlsn) locally, with disabled TLS in the config file.

   ```yaml
   # congig.yaml
   tls:
     enabled: false
   ```

   ```bash
   cargo run --release -- --config-file ./config/config.yaml
   ```

2. Run the example

   ```bash
   cargo run --relese
   ```

   Sometimes the example hangs when one of Prover instances enters into an endless loop at the key exchange phase.

   Run the example several times. It's highly likely it will get hanging within 10 attempts.
