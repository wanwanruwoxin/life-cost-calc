import { createApp } from "vue";
import App from "./App.vue";
import "virtual:uno.css";
import { Quasar } from "quasar";

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'

// Import Quasar css
import 'quasar/src/css/index.sass'

const app = createApp(App)

app.use(Quasar, {
  plugins: {}, // import Quasar plugins and add here
});

app.mount("#app");
