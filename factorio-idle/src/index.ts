//@ts-ignore
import init, { greet } from 'game-core';
// Don't worry if vscode told you can't find game-core
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install game-core for you

init().then(() => {
  console.log('init wasm-pack');
  greet('from vite!');
});
