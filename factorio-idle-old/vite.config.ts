import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';
import react from '@vitejs/plugin-react-swc'
export default defineConfig({
  build: {
    minify: false
  },
  plugins: [react(), wasmPack(['./game-core'])]
});
