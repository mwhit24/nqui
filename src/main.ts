import {createApp} from "vue";
import "./style.css";
import App from "./App.vue";
import {createPinia} from 'pinia'
import PrimeVue from 'primevue/config';
import "primevue/resources/themes/saga-blue/theme.css"; //theme
import "primevue/resources/primevue.min.css"; //core CSS
import "primeicons/primeicons.css"; //icons

const pinia = createPinia()
createApp(App).use(pinia).use(PrimeVue).mount("#app");
