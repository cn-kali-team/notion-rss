/**
 * plugins/index.js
 *
 * Automatically included in `./src/main.js`
 */

// Plugins
import {vuetify,i18n} from './vuetify'
export function registerPlugins (app) {
  app.use(vuetify)
  app.use(i18n)
}
