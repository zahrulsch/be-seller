import { createApp } from "vue";
import { routes } from "./routes";
import App from "./App.vue";
import "./style.scss";
import "ant-design-vue/dist/antd.css"

createApp(App).use(routes).mount("#app");
