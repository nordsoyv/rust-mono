const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
// const paths = require('./config/paths');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  module : {
    rules : [
      // transpile js
      {
        test: /\.(js)x?$/,
        exclude: /node_modules/,
        // include: paths.src,
        loader: 'babel-loader',
        // options: babelConfig,
      },
   ]
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
