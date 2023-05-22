wasm-pack build --target bundler
cd pkg
npm link
npm install
cd ../site
npm link module-import
npm install
npm run serve
