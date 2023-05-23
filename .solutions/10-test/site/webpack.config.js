const path = require("path");
module.exports = {
    cache: false,
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "./index.js",
    },
    mode: "development",
    experiments: {
        asyncWebAssembly: true,
    },
},
devServer = {
  headers: {
    'Cache-Control': 'no-store',
  },
}
