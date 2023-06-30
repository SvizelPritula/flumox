import { defineConfig, Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { minify } from 'html-minifier-terser';
import { parse as parseYaml } from "yaml";
import { join as joinPath } from 'path';

export default defineConfig((mode) => ({
  plugins: [
    svelte({
      compilerOptions: mode.command === "build" ? {
        cssHash: (({ hash, css }) => `_${hash(css)}`)
      } : {}
    }),
    minifyHtml(),
    translate('cs')
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
