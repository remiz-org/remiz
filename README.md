# Remiz

<p align="center">
  <img src="https://github.com/remiz-org/remiz/blob/main/logo.png?raw=true">
</p>

<p align="center">
  <a href="#" alt="maintenance-status"><img src="https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg"/></a>
  <a href="#" alt="integration"><img src="https://github.com/remiz-org/remiz/actions/workflows/test.yml/badge.svg"/></a>
  <a href="https://remiz-org.github.io/remiz/", alt="Website/Documentation"><img src="https://img.shields.io/badge/Documentation-OK-15afff" /></a>
  <a href="https://opensource.org/licenses/MIT" alt="License: MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg"/></a>
  <a href="https://github.com/remiz-org/remiz/releases/latest" alt="Github All Releases"><img src="https://img.shields.io/github/downloads/remiz-org/remiz/total.svg"/></a>
</p>

## What is Remiz ?

Remiz is a simple (but extensible) command line tool to create and deploy reproducible copy of your projects.
All the data is serialized into a single compressed binary file ending with ".pack" (customizable).

This tool could be used inside a CI/CD pipeline (Gitlab, Jenkins, ...) to describe the packaging and deployment process. You can also use it on a local machine whether the project is versioned or not.

Remiz is open source (MIT licensed), cross platform, fast and fully customizable with TOML configuration files.

## How to get started

The best place to learn about Remiz is by browsing the documentation website : [https://remiz-org.github.io/remiz/](https://remiz-org.github.io/remiz/).

## About the .pack file format

Remiz format is based on the open Multi-Layer Archive ([MLA](https://github.com/ANSSI-FR/MLA)). Thus, .pack file format supports compression and encryption out of the box.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](LICENSE).

## Disclaimer
This SOFTWARE PRODUCT is provided by THE PROVIDER "as is" and "with all faults."
THE PROVIDER makes no representations or warranties of any kind concerning the
safety, suitability, lack of viruses, inaccuracies, typographical errors, or
other harmful components of this SOFTWARE PRODUCT. There are inherent dangers
in the use of any software, and you are solely responsible for determining
whether this SOFTWARE PRODUCT is compatible with your equipment and other
software installed on your equipment. You are also solely responsible for the
protection of your equipment and backup of your data, and THE PROVIDER will not
be liable for any damages you may suffer in connection with using, modifying,
or distributing this SOFTWARE PRODUCT.
