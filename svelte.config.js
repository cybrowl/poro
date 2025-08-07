import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const config = {
  preprocess: vitePreprocess(),
  kit: {
    files: {
      assets: "src/ui/assets",
      lib: "src/ui/lib",
      routes: "src/ui/routes",
      appTemplate: "src/ui/app.html",
    },
    alias: {
      $components: "src/ui/components",
    },
    adapter: adapter({
      pages: "build",
      assets: "build",
      fallback: "index.html",
      precompress: true,
    }),
  },
};

export default config;
