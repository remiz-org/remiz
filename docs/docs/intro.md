---
sidebar_position: 1
---

# About Remiz

Let's discover in this page what you can do with **Remiz**.

## Introduction

Remiz is a simple (but extensible) command line tool to create and deploy reproducible copy of your projects. All the data is serialized into a single compressed binary file ending with ".pack" (customizable).

This tool could be used inside a CI/CD pipeline (Gitlab, Jenkins, ...) to describe the packaging and deployment process or locally on a non versionned project.

Remiz is open source (MIT licensed), cross platform, fast and fully customizable with TOML configurations file.


### The .pack format

Remiz stands on Multi Layer Archive format (more info : [MLA](https://github.com/ANSSI-FR/MLA)).
It's based on Brotli compression (developped by the Dropbox team) and supports encryption.
MLA is developped by the ANSSI (French National Agency for the Security of Information Systems).


