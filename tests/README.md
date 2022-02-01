# Integrations tests

## Overview

Whats is done:

1. Construct an environment with several files, a TOML package configuration
   file and a TOML global configuration file.

2. Build a .pack of the project.

3. Compare the hash of the .pack with the precomputed tested package.

4. Deploy the .pack

5. Compare the hash of the deployed folder with the original folder.
