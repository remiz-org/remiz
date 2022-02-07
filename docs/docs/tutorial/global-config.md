---
sidebar_position: 2
---

# Set up global configuration

The global configuration file is required for Remiz to work.
You can specify the location of the file using the -g/--global_config_file command flag. If not specified, Remiz will look for the configuration file to be put in the same directory as the binary.

A configuration file includes at 2 sections:

- all **packagers** to use when building/deploying
- all **stores** in each you want to store/deploy from the artifact file.

## Create your first global configuration

Create a toml file at where the binary is located, for example `remiz/configuration.toml`:

```md title="configuration.toml"
[packagers]
copy_files = "packagers/copy_files.py"
bar = "packagers/bar.py"

[store]
local = "packages/{name}_v{version}.pack"
```

A new document is now available at `http://localhost:3000/docs/hello`.

## Configure the Sidebar

Docusaurus automatically **creates a sidebar** from the `docs` folder.

Add metadata to customize the sidebar label and position:

```md title="docs/hello.md" {1-4}
---
sidebar_label: 'Hi!'
sidebar_position: 3
---

# Hello

This is my **first Docusaurus document**!
```

It is also possible to create your sidebar explicitly in `sidebars.js`:

```diff title="sidebars.js"
module.exports = {
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Tutorial',
-     items: [...],
+     items: ['hello'],
    },
  ],
};
```
