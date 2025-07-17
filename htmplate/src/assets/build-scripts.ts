import * as path from "jsr:@std/path@1.1.1";
import { parseArgs } from "jsr:@std/cli@1.0.20";
import ts from "npm:typescript@5.8.3";
import UglifyJs from "npm:uglify-js@3.19.3";

const args = parseArgs(Deno.args, {
  string: ["outdir", "indir"],
});

const indir = args.indir ?? "htmplate/src/assets/scripts/";
const outdir = args.outdir ?? "htmplate/src/assets/scripts/dist/";

Deno.mkdirSync(outdir, { recursive: true });

for (const dirEntry of Deno.readDirSync(indir)) {
  if (!dirEntry.isFile || !dirEntry.name.endsWith(".ts")) {
    continue;
  }

  compile_typescript(path.join(indir, dirEntry.name), outdir);
}

function compile_typescript(source_path: string, output_directory: string) {
  const denoComments = ["// deno-lint-ignore-file", "// deno-fmt-ignore-file", "// @ts-nocheck"];

  const contents = Deno.readTextFileSync(source_path);

  const js = ts.transpileModule(contents, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2021,
    },
  });
  const dts = ts.transpileDeclaration(contents, {});

  const stem = path.basename(source_path, ".ts");

  const jsHeader = [...denoComments, `// @ts-self-types="./${stem}.d.ts`].join("\n");
  const jsBody = js.outputText;
  const outputJs = UglifyJs.minify(`${jsHeader}\n${jsBody}`, {
    output: { comments: "all" },
    mangle: { keep_fargs: true },
  }).code;

  const dtsHeader = denoComments.join("\n");
  const dtsBody = dts.outputText.replaceAll('.ts"', '.d.ts"');
  const outputDts = `${dtsHeader}\n${dtsBody}`;

  Deno.writeTextFileSync(path.join(output_directory, `${stem}.js`), outputJs);
  Deno.writeTextFileSync(path.join(output_directory, `${stem}.d.ts`), outputDts);
}
