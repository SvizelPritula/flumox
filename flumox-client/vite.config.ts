import { defineConfig, Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { minify } from 'html-minifier-terser';

export default defineConfig((mode) => ({
  plugins: [
    svelte({
      compilerOptions: mode.command === "build" ? {
        cssHash: (({ hash, css }) => `_${hash(css)}`)
      } : {}
    }),
    minifyHtml()
  ],
  server: {
    proxy: {
      "/api": "http://localhost:8000/"
    }
  }
}));

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
