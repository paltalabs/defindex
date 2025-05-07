/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ["@repo/eslint-config/next.js", "next", "next/typescript"],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: true,
  },
  env: {
    jest: true, // Add this line
  },
};
