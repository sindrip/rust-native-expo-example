const { getDefaultConfig } = require("expo/metro-config");
const path = require("node:path");

const config = getDefaultConfig(__dirname);

// Watch the parent module directory for changes during development
config.watchFolders = [path.resolve(__dirname, "..")];

// Prevent duplicate React/React Native from the parent module's node_modules
config.resolver.blockList = [
  /\.\.\/node_modules\/react\/.*/,
  /\.\.\/node_modules\/react-native\/.*/,
];

// Resolve modules from both example and parent node_modules
config.resolver.nodeModulesPaths = [
  path.resolve(__dirname, "node_modules"),
  path.resolve(__dirname, "..", "node_modules"),
];

// Map the module name to the parent directory
config.resolver.extraNodeModules = {
  "app-mobile": path.resolve(__dirname, ".."),
};

module.exports = config;
