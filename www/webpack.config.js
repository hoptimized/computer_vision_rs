const path = require('path');

const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const WorkboxPlugin = require('workbox-webpack-plugin');

module.exports = {
    entry: path.resolve(__dirname, "index.js"),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".."),
            outDir: path.resolve(__dirname, "pkg/frontend"),
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../backend"),
            outDir: path.resolve(__dirname, "pkg/backend"),
        }),
        new WorkboxPlugin.GenerateSW({
            clientsClaim: true,
            skipWaiting: true,
            maximumFileSizeToCacheInBytes: 10000000,
        }),
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "index.html"),
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
