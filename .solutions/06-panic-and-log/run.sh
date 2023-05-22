wasm-pack build --target bundler
cd pkg
npm link
npm install
cd ../site
npm link panic-and-log
npm install
npm run serve
