import { defineConfig, Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { ManifestOptions, VitePWA } from "vite-plugin-pwa";
import { minify } from 'html-minifier-terser';
import { parse as parseYaml } from "yaml";
import { join as joinPath } from 'path';

const language = 'cs';

const manifest: Partial<ManifestOptions> = {
  name: "Flumox",
  short_name: "Flumox",
  description: "An app for playing outside games",
  theme_color: "#262626",
  background_color: "#0d0d0d",
  lang: language,
  icons: [],
};

export default defineConfig((mode) => ({
  plugins: [
    svelte({
      compilerOptions: mode.command === "build" ? {
        cssHash: (({ hash, css }) => `_${hash(css)}`)
      } : {}
    }),
    VitePWA({
      strategies: "generateSW",
      registerType: 'autoUpdate',
      injectRegister: null,
      filename: "worker.js",
      workbox: {
        inlineWorkboxRuntime: true
      },
      manifest
    }),
    minifyHtml(),
    translate(language)
  ],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8000/",
        ws: true
      }
    }
  },
  build: {
    modulePreload: { polyfill: false }
  }
}));

function translate(lang: string): Plugin {
  const moduleName = "$translations";
  const moduleId = `\0${moduleName}`;
  const path = joinPath(process.cwd(), 'src/translations', `${lang}.yaml`);

  return {
    name: "translate",
    resolveId(source) {
      if (source == moduleName)
        return moduleId;
    },
    load(id) {
      if (id == moduleId) {
        return `export * from ${JSON.stringify(path)};\n`
      }
    },
    transform(code, id) {
      if (id.endsWith(".yaml")) {
        let payload = parseYaml(code);

        let result = "";

        for (let key in payload) {
          let value = payload[key];

          if (typeof value != "string")
            throw new Error("Value must be string");

          result += `export const ${key} = ${JSON.stringify(value)};\n`;
        }

        return result;
      }
    },
    transformIndexHtml(html) {
      return html.replace("{{lang}}", lang);
    }
  };
}

function minifyHtml(): Plugin {
  return {
    name: "minify-html",
    apply: "build",
    transformIndexHtml: {
      order: "post",
      handler(html) {
        return minify(html, {
          collapseWhitespace: true,
          decodeEntities: true,
        });
      }
    }
  };
}
