import { defineConfig, Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { ManifestOptions, VitePWA } from "vite-plugin-pwa";
import { minify } from 'html-minifier-terser';
import { parse as parseYaml } from "yaml";
import { join as joinPath } from 'path';
import { readFile } from 'fs/promises';

export default defineConfig(async (mode) => {
  const language = 'cs';
  const translations = await loadTranslations(language);

  const manifest: Partial<ManifestOptions> = {
    name: translations.appName,
    short_name: translations.appName,
    description: translations.appDescription,
    theme_color: "#262626",
    background_color: "#0d0d0d",
    lang: language,
    icons: [],
  };

  return {
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
      translate(language),
      templateHtml({
        "{{lang}}": language,
        "{{name}}": translations.appName,
        "{{description}}": translations.appDescription,
        "{{enable-js}}": translations.noscript,
      })
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
  };
});

function getTranslationsPath(lang: string): string {
  return joinPath(process.cwd(), 'src/translations', `${lang}.yaml`);
}

async function loadTranslations(lang: string): Promise<Record<string, string>> {
  let content = await readFile(getTranslationsPath(lang), { encoding: "utf-8" });
  return parseYaml(content);
}

function translate(lang: string): Plugin {
  const moduleName = "$translations";
  const moduleId = `\0${moduleName}`;
  const path = getTranslationsPath(lang);

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
    }
  };
}

function templateHtml(replacements: Record<string, string>): Plugin {
  return {
    name: "template-html",
    transformIndexHtml(html) {
      for (let key in replacements) {
        html = html.replace(key, replacements[key]);
      }

      return html;
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
