const webpack = require("webpack");
const path = require("path");
const HtmlPlugin = require('html-webpack-plugin')
const CopyPlugin = require('copy-webpack-plugin')
const WriteFilePlugin = require('write-file-webpack-plugin')

const plugins = [
	new webpack.LoaderOptionsPlugin({
		options: {
			tslint: {
				emitErrors: true,
				failOnHint: true
			}
		}
	}),
	new HtmlPlugin({
		template: 'src/index.html',
		inject: false
	}),
	new WriteFilePlugin(),
	new CopyPlugin([
		// { from: 'lib', to: 'lib' },
		{ from: 'assets', to: '.' }
	])
];

var config = {
	mode: "development",
	devtool: "source-map",
	context: path.resolve("./"),
	entry: {
		app: "./src/bootstrap.js"
	},
	output: {
		path: path.resolve("./dist"),
		filename: "index.js",
		sourceMapFilename: "index.map",
		devtoolModuleFilenameTemplate: function (info) {
			return "file:///" + info.absoluteResourcePath;
		}
	},
	output: {
		path: path.resolve(__dirname, 'dist'),
		filename: 'index.js'
	},
	module: {
		rules: [
			{
				enforce: "pre",
				test: /\.tsx?$/,
				exclude: /\/node_modules\//,
				use: ["awesome-typescript-loader", "source-map-loader"]
			}
		]
	},
	resolve: {
		extensions: [".ts", ".js", ".js", ".jsx", ".wasm", ".json"]
	},
	plugins: plugins,
	devServer: {
		contentBase: path.join(__dirname, '/dist'),
		compress: true,
		port: 5140,
		host: "0.0.0.0",
		hot: true,
		disableHostCheck: true,
		watchContentBase: true,
	}
};

module.exports = config;
