const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: path.resolve(__dirname, "index.js"),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "index.html"),
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".."),
            outDir: path.resolve(__dirname, "pkg"),
        }),
        new CopyWebpackPlugin({
            patterns: [
                {
                    from: path.resolve(__dirname, "static"),
                    to: path.resolve(__dirname, "dist")
                },
            ]
        }),
    ],
    devServer: {
        static: {
            directory: path.join(__dirname, 'dist'),
        },
        port: 8080,
    },
    mode: 'production',
    stats: 'minimal',
    experiments: {
        asyncWebAssembly: true
    }
};
