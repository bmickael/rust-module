wasm-pack build --target bundler
cd pkg
npm link
cd ../site
npm link rust-structure
npm install
npm run serve
