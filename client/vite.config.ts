import { defineConfig, Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { minify } from 'html-minifier-terser';

export default defineConfig({
  plugins: [svelte(), minifyHtml()],
  server: {
    proxy: {
      "/api": "http://localhost:3000/"
    }
  }
});

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
