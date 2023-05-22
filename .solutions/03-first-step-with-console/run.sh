wasm-pack build --target bundler
cd pkg
npm link
cd ../site
npm link first-step-with-console
npm install
npm run serve
