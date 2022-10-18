const HtmlWebpackPlugin = require("html-webpack-plugin");
const ReactRefreshPlugin = require("@pmmmwh/react-refresh-webpack-plugin");
const NodePolyfillPlugin = require("node-polyfill-webpack-plugin");

const path = require("path");
const child_process = require("child_process");
const webpack = require("webpack");

module.exports = (env) => {
	console.log("Production: ", env.production); // true

	const __versionString__ = child_process
		.execSync("git rev-parse --short HEAD")
		.toString();
	console.log("__versionString__", __versionString__);

	const deploy_dir = path.join(__dirname, "../deploy", "www");

	const appEnv = process.env.ENV ? process.env.ENV : "nightly";
        console.log("ENV", appEnv, process.env.ENV_TYPE);

        const env_type = process.env.ENV_TYPE ? process.env.ENV_TYPE.trim() : "dev";
        console.log("current channel", env_type);

    
	const plugins = [
		new NodePolyfillPlugin(),
		new webpack.DefinePlugin({
			__VERSION__: JSON.stringify(__versionString__),
			__ENV__: JSON.stringify(appEnv),
		}),
		new HtmlWebpackPlugin({
			title: "cyfs-git",
			template: "./src/index.html",
		}),
		new webpack.DefinePlugin({
			// 传递 process.env.ENV_TYPE
			"process.env.ENV_TYPE": JSON.stringify(env_type),
		}),
	];

	// dev模式 add refresh
	if (!env.production) {
		plugins.unshift(new ReactRefreshPlugin());
	}

	return {
		entry: ["./src/App.tsx"],
		output: {
			filename: "bundle.js",
			path: deploy_dir,
		},
		mode: env.production ? "production" : "development",
		devtool: env.production ? false : "source-map",
		resolve: {
			extensions: [".ts", ".tsx", ".js", ".json"],
			alias: {
				"@src": path.resolve(__dirname, "src"),
				"@root": path.resolve(__dirname, "../"),
			},
		},
		devServer: {
			static: path.join(__dirname, "./src"),
			port: 8080,
		},
		module: {
			rules: [
				{
					test: /\.tsx?|jsx$/,
					loader: "esbuild-loader",
					options: {
						loader: "tsx",
						target: "chrome80",
						tsconfigRaw: require("./tsconfig.json"),
					},
				},
				{
					test: /\.(css|less)$/,
					use: [
						"style-loader",
						{
							loader: "css-loader",
							options: {
								modules: {
									exportLocalsConvention: "camelCaseOnly",
								},
							},
						},
						{
							loader: "esbuild-loader",
							options: {
								loader: "css",
								minify: env.production,
							},
						},
						"less-loader",
						{
							loader: "style-resources-loader",
							options: {
								patterns: path.resolve(__dirname, "src/styles/common.less"), //全局引入公共的less 文件
							},
						},
					],
				},
				{
					test: /\.(jpg|png|svg|ico|icns)$/,
					loader: "file-loader",
					options: {
						name: "[hash:10].[ext]",
					},
				},
			],
		},
		plugins: plugins,
		externals: [
			{
				react: "React",
				"react-dom": "ReactDOM",
				"react-router-dom": "ReactRouterDOM",
				antd: "antd",
				// fs: require("fs"),
				// 'lodash': 'lodash',
			},
			function ({ context, request }, callback) {
				// console.log('externals', request)
				if (/cyfs-sdk$/.test(request)) {
					console.log("replace", request);
					return callback(null, "cyfs");
				}
				callback();
			},
		],
	};
};
