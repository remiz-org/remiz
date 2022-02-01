---
sidebar_position: 5
---

# Deploy your site

Remiz is a **static-site-generator** (also called **[Jamstack](https://jamstack.org/)**).

*Packaging/Deploying CLI*

## Build your site

Build your site **for production**:

```bash
npm run build
```

The static files are generated in the `build` folder.

## Deploy your site

Test your production build locally:

```bash
npm run serve
```

The `build` folder is now served at `http://localhost:3000/`.

You can now deploy the `build` folder **almost anywhere** easily, **for free** or very small cost (read the **[Deployment Guide](https://Remiz.io/docs/deployment)**).
