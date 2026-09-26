// The package source imports its siblings without an extension, as a bundler
// expects; Node's type stripping does not add one. This hook retries such an
// import with `.ts`, so the drawings read `src/field` as it stands.
// Run: node --import ./video/draw/ts-resolve.ts <script>
import { registerHooks } from 'node:module';

registerHooks({
  resolve(specifier, context, next) {
    try {
      return next(specifier, context);
    } catch (error) {
      if (!specifier.startsWith('.') || /\.[a-z]+$/.test(specifier)) throw error;
      return next(`${specifier}.ts`, context);
    }
  },
});
