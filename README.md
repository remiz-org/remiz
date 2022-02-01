# Remiz

<p align="center">
  <img src="https://github.com/remiz-org/remiz/blob/main/logo.png?raw=true">
</p>

![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![integration](https://github.com/remiz-org/remiz/actions/workflows/test.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Github All Releases](https://img.shields.io/github/downloads/remiz-org/remiz/total.svg)](https://github.com/remiz-org/remiz/releases/latest)

## What is Remiz ?

Remiz is a simple (but extensible) command line tool to create and deploy reproducible copy of your projects.
All the data is serialized into a single compressed binary file ending with ".pack" (customizable).

This tool could be used inside a CI/CD pipeline (Gitlab, Jenkins, ...) to describe the packaging and deployment process or locally on a non versionned project.

Remiz is open source (MIT licensed), cross platform, fast and fully customizable with TOML configurations file.


## About the .pack file format

Remiz format is based on the Multi-Layer Archive ([MLA](https://github.com/ANSSI-FR/MLA)). Thus, .pack file format supports compression and encryption out of the box.
