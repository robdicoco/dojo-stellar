// import './assets/main.css'
import '@fortawesome/fontawesome-free/css/all.min.css'; // Import Font Awesome
import './assets/style.css';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import router from './router';
const app = createApp(App);
// Set initial theme based on user preference or browser setting
const savedTheme = localStorage.getItem('theme') || 'dark';
document.documentElement.setAttribute('data-theme', savedTheme);
app.use(createPinia());
app.use(router);
app.mount('#app');
// Watch for theme changes and save to local storage
app.config.globalProperties.$watchTheme = (newTheme) => {
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
};
// Pass the toggleTheme function to the root component
app.config.globalProperties.$toggleTheme = () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    app.config.globalProperties.$watchTheme(newTheme);
};
