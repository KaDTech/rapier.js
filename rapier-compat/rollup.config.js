const { base64 } = require("rollup-plugin-base64");
import { terser } from 'rollup-plugin-terser';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from "@rollup/plugin-commonjs";
import typescript from "rollup-plugin-typescript2";
import filesize from "rollup-plugin-filesize";
import copy from "rollup-plugin-copy";
import path from "path";

const config = (dim) => {
  const includedPaths = [
    `./gen${dim}/**/*.ts`,
    path.resolve(__dirname, `gen${dim}/**/*.ts`),
  `./gen${dim}/*.ts`,
  path.resolve(__dirname, `gen${dim}/*.ts`),
  `./src${dim}/*`,
  path.resolve(__dirname, `src${dim}/*`)
];
  console.log(`dim=${dim} __dirname=${__dirname}`);
  console.log(`includedPaths=${includedPaths}`);

return {
  input: `./gen${dim}/rapier.ts`,
  output: [
    {
      file: `pkg${dim}/rapier.es.js`,
      format: "es",
      sourcemap: true,
      exports: 'named',
    },
    {
      file: `pkg${dim}/rapier.cjs.js`,
      format: "cjs",
      sourcemap: true,
      exports: 'named',
    },
  ],
  plugins: [
    copy({
      targets: [
        {
          src: `../rapier${dim}/pkg/package.json`,
          dest: `./pkg${dim}/`,
          transform(content) {
            let config = JSON.parse(content.toString());
            config.name = `@kadtech/kadtechrapier${dim}-compat`;
            config.description +=
              " Compatibility package with inlined webassembly as base64.";
            config.types = "rapier.d.ts";
            config.main = "rapier.cjs.js";
            config.module = "rapier.es.js";
            // delete config.module;
            config.files = ["*"];
            return JSON.stringify(config, undefined, 2);
          },
        },
        {
          src: `../rapier${dim}/LICENSE`,
          dest: `./pkg${dim}`,
        },
        {
          src: `../rapier${dim}/README.md`,
          dest: `./pkg${dim}`,
        },
      ],
    }),
    base64({ include: "**/*.wasm" }),
    terser(),
    nodeResolve(),
    commonjs(),
    typescript({
      tsconfig: path.resolve(__dirname, `tsconfig.pkg${dim}.json`),
      include: includedPaths,
      sourceMap: true,
      inlineSources: true,
    }),
    filesize(),
  ],
}};

export default [config("2d"), config("3d")];
