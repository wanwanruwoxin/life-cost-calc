import "virtual:uno.css";
// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
// Import Quasar css
import 'quasar/src/css/index.sass'
// Import global styles
import './assets/global.css'
import { createApp } from "vue";
import { Quasar } from "quasar";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./routes";

const pinia = createPinia()

const app = createApp(App)
app.use(pinia);
app.use(router);

app.use(Quasar, {
  plugins: {}, // import Quasar plugins and add here
});

app.mount("#app");
