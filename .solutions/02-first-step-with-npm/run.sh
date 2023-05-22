wasm-pack build --target bundler
cd pkg
npm link
cd ../../02-first-step-with-npm-site
npm link first-step-with-npm
npm install
npm run serve
