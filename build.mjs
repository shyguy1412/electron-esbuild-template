import { context } from 'esbuild';
import { readFile } from 'fs/promises';

const WATCH = process.argv.includes('--watch');
const HOST = process.argv[(process.argv.indexOf('--host') + 1) || -1] ?? "0.0.0.0";
const PORT = +(process.argv[(process.argv.indexOf('--port') + 1) || -1] ?? 8000);

const createMainContext = async () => await context({
  entryPoints: [
    "src/main/main.ts",
    "src/main/preload.ts",
  ],
  outdir: "./build",
  outbase: "./src/main",
  loader: {
    ".node": "copy"
  },
  bundle: true,
  packages: "external",
  format: 'cjs',
  platform: "node",
  logLevel: 'info'
});

/** @type import("esbuild").Plugin */
const reloadPlugin = {
  name: "HTMLPlugin",
  setup(pluginBuild) {
    pluginBuild.onLoad({ filter: /.*\.html$/ }, async (opts) => {
      const file = await readFile(opts.path, { encoding: "utf8" });
      return {
        contents: file.replace("</head>", "    <script>new EventSource('/esbuild').addEventListener('change', () => location.reload())</script>\n</head>"),
        loader: "copy"
      };
    });
  }
};

const createRenderContext = async () => await context({
  entryPoints: [
    "src/render/index.tsx",
    "src/render/index.html"
  ],
  loader: { ".html": "copy", ".woff2": "copy" },
  plugins: WATCH ? [reloadPlugin] : [],
  outbase: "./src/render",
  outdir: "./build",
  bundle: true,
  format: 'esm',
  platform: 'browser',
  alias: {
    "react": "preact/compat",
    "react-dom": "preact/compat",
    "react-reconciler": "preact-reconciler",
  },
  // minify: !WATCH,
  logLevel: 'info',
});

const renderCtx = await createRenderContext();
const mainCtx = await createMainContext();

if (WATCH) {

  renderCtx.serve({ host: HOST, port: PORT });
  renderCtx.watch();
  mainCtx.watch();

} else {

  await renderCtx.rebuild();
  renderCtx.dispose();

  await mainCtx.rebuild();
  mainCtx.dispose();

}

