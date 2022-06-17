const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: './src/main.ts',
  experiments: {
    asyncWebAssembly: true,
  },
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.css/i,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.ts/i,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js', '.css'],
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'Reversi',
      template: path.resolve(__dirname, 'src/index.html'),
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'reversi-agent'),
      outDir: path.resolve(__dirname, 'reversi-agent/pkg'),
    }),
  ],
};
