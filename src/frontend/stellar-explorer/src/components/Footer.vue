<template>
  <footer class="footer">
    <div class="footer-content">
      <a
        v-for="(link, index) in links"
        :key="index"
        :href="link.url"
        target="_blank"
        class="footer-link"
      >
        <sub
          ><b>{{ link.name }}</b></sub
        >
      </a>
      <span class="separator">|</span>
      <span class="source-code">
        Source Code:
        <a href="https://github.com/robdicoco/dojo-stellar" target="_blank">dojo-stellar</a>
        | License: MIT
      </span>
    </div>
  </footer>
</template>

<script>
export default {
  data() {
    return {
      links: [
        { name: 'robdicoco', url: 'https://github.com/robdicoco' },
        { name: 'alfatektecnologia', url: 'https://github.com/alfatektecnologia' },
        { name: 'lucenfort', url: 'https://github.com/lucenfort' },
        { name: 'uederson-ferreira', url: 'https://github.com/uederson-ferreira' },
      ],
    }
  },
}
</script>

<style scoped>
.footer {
  background-color: var(--background-color);
  color: var(--text-color);
  padding: 10px 20px;
  text-align: center;
  font-size: 14px;
  position: fixed;
  bottom: 0;
  width: 100%;
  box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.2);
}

.footer-content {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: nowrap;
}

.footer-link {
  margin: 0 10px;
  text-decoration: none;
  color: var(--text-color);
  transition:
    transform 0.3s ease,
    color 0.3s ease;
}

.footer-link:hover {
  transform: scale(1.2);
  color: #42b983; /* Vue green color */
}

.separator {
  margin: 0 10px;
  color: #42b983;
}

.source-code {
  margin-left: 10px;
  font-size: 12px;
}

.source-code a {
  color: #42b983;
  text-decoration: none;
  transition: color 0.3s ease;
}

.source-code a:hover {
  color: #6dd4a8;
}
</style>
