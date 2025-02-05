<template>
  <header class="header">
    <!-- Left Section: Logo and Navigation Links -->
    <div class="header-left">
      <div class="logo">
        <span>Lumen | Explorer</span>
      </div>
      <nav class="navigation">
        <ul>
          <li><a href="#">Blockchain</a></li>
          <li><a href="#">Accounts</a></li>
          <li><a href="#">Market</a></li>
          <li><a href="#">Assets</a></li>
          <li><a href="#">Charts</a></li>
        </ul>
      </nav>
    </div>

    <!-- Right Section: Favorites, Network Selection, and Settings -->
    <div class="header-right">
      <button class="favorites-btn" @click="toggleFavorite">
        <i
          :class="['fas', isFavorite ? 'fa-heart' : 'fa-heart']"
          :style="{ color: isFavorite ? 'red' : 'blue' }"
        ></i>
      </button>
      <div class="network-selector">
        <select v-model="selectedNetwork">
          <option value="LIVENET">LIVENET</option>
          <option value="TESTNET">TESTNET</option>
          <option selected="true" value="LOCAL">LOCAL</option>
        </select>
      </div>
      <button class="theme-toggle-btn" @click="toggleTheme">
        <i :class="theme === 'dark' ? 'fas fa-lightbulb' : 'far fa-lightbulb'"></i>
      </button>
    </div>
  </header>
</template>

<script>
export default {
  name: 'Header',
  data() {
    return {
      selectedNetwork: 'LOCAL', // Default selected network
      theme: 'dark', // Default theme
      isFavorite: false, // Track if the current page is in favorites
    }
  },
  methods: {
    toggleTheme() {
      this.theme = this.theme === 'dark' ? 'light' : 'dark'
      document.documentElement.setAttribute('data-theme', this.theme)
    },
    toggleFavorite() {
      const favorites = JSON.parse(localStorage.getItem('favorites')) || []
      const currentPageUrl = window.location.href

      if (this.isFavorite) {
        // Remove from favorites
        this.isFavorite = false
        favorites.splice(favorites.indexOf(currentPageUrl), 1)
      } else {
        // Add to favorites
        this.isFavorite = true
        favorites.push(currentPageUrl)
      }

      localStorage.setItem('favorites', JSON.stringify(favorites))
    },
  },
}
</script>

<style scoped>
/* General Header Styles */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--background-color);
  color: var(--text-color);
  padding: 10px 20px;
}

.header-left {
  display: inline-flex;
  justify-content: first baseline;
  align-items: left;
  vertical-align: center;
  background-color: var(--background-color);
  color: var(--text-color);
  padding: 10px 20px;
}

.header-right {
  display: inline-flex;
  justify-content: first baseline;
  align-items: right;
  vertical-align: top;
  background-color: var(--background-color);
  color: var(--text-color);
  padding: 10px 20px;
}

.logo {
  font-size: 18px;
  font-weight: bold;
  margin-right: 20px;
}

.navigation ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
}

.navigation li {
  margin-right: 15px;
}

.navigation a {
  color: var(--text-color);
  text-decoration: none;
  font-size: 14px;
  transition: color 0.3s ease;
}

.network-selector select {
  background-color: var(--background-color);
  color: var(--text-color);
  border: none;
  padding: 5px 10px;
  border-radius: 5px;
  font-size: 14px;
  cursor: pointer;
  outline: none;
}

.navigation a:hover {
  color: #00bcd4;
}

.favorites-btn,
.settings-btn {
  background-color: var(--background-color);
  border: none;
  color: var(--text-color);
  font-size: 18px;
  cursor: pointer;
  margin-left: 10px;
  transition: color 0.3s ease;
}

.favorites-btn:hover,
.settings-btn:hover {
  color: #00bcd4; /* Highlight color on hover */
}

.network-selector select:focus {
  box-shadow: 0 0 5px rgba(0, 188, 212, 0.5);
}
</style>
