import { dtsPlugin } from "npm:esbuild-plugin-d.ts";
import { build } from "npm:esbuild";
import { parseArgs } from "jsr:@std/cli";

const args = parseArgs(Deno.args, {
  string: ["outdir", "entryPoint"],
});

const declarations = [
  '// @ts-self-types="./lib.d.ts"',
  "// deno-lint-ignore-file",
  "// deno-fmt-ignore-file",
  "// @ts-nocheck",
];

const entryPoint = (args.entryPoint ?? "htmplate/src/assets/scripts/lib.ts").replaceAll("\\", "/");
const outdir = (args.outdir ?? "htmplate/src/assets/scripts/dist").replaceAll("\\", "/");

console.log(`outdir: ${outdir}`);
console.log(`entryPoint: ${entryPoint}`);

await build({
  entryPoints: [entryPoint],
  outdir: outdir,
  plugins: [dtsPlugin({
    experimentalBundling: true,
  })],
  target: "ES2021",
  platform: "neutral",
  format: "esm",
  minify: true,
  banner: {
    "js": declarations.join("\n"),
  },
  bundle: true,
});
